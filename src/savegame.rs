use itertools::izip;
use thiserror::Error;

/// Contents of an Etterna ReplayV2 replay file. See [`parse_replay_v2_fast`] for more
#[derive(Debug, Clone, PartialEq)]
pub struct ReplayV2Fast {
	pub notes: Vec<ReplayV2Note>,
	pub num_mine_hits: u32,
	pub num_hold_drops: u32,
}

impl ReplayV2Fast {
	/// The returned vectors are sorted chronologically by the **note time**.
	pub fn split_into_lanes(
		&self,
		timing_info: &crate::TimingInfo,
		rate: crate::Rate,
	) -> ([Vec<f32>; 4], [Vec<f32>; 4]) {
		let unsorted_ticks: Vec<u32> = self.notes.iter().map(|n| n.tick).collect();
		let permutation = permutation::sort(&unsorted_ticks[..]);

		let ticks = permutation.apply_slice(&unsorted_ticks[..]);
		let note_seconds = timing_info.ticks_to_seconds(&ticks);

		let notes = permutation.apply_slice(&self.notes[..]); // sorted by note

		let mut note_seconds_columns = [vec![], vec![], vec![], vec![]];
		let mut hit_seconds_columns = [vec![], vec![], vec![], vec![]];

		for (note_second, note) in izip!(note_seconds, notes) {
			if note.column >= 4 {
				continue;
			}

			let note_second = note_second / rate.as_f32();

			note_seconds_columns[note.column as usize].push(note_second);
			if let crate::Hit::Hit { deviation } = note.hit {
				hit_seconds_columns[note.column as usize].push(note_second + deviation);
			}
		}

		(note_seconds_columns, hit_seconds_columns)
	}
}

impl crate::SimpleReplay for ReplayV2Fast {
	fn iter_hits(&self) -> Box<dyn '_ + Iterator<Item = crate::Hit>> {
		Box::new(self.notes.iter().map(|n| n.hit))
	}
}

/// Represents a single note in a v2 replay
#[derive(Debug, Clone, PartialEq)]
pub struct ReplayV2Note {
	pub tick: u32,
	pub hit: crate::Hit,
	pub column: u8,
}

#[derive(Debug, Error)]
pub enum ReplayParseError {
	#[error("Replay file line {line_num} had no contain tick information")]
	MissingTick { line_num: usize },
	#[error("Replay file line {line_num} had no contain deviation information")]
	MissingDeviation { line_num: usize },
	#[error("Replay file line {line_num} had no contain note lane information")]
	MissingLane { line_num: usize },
}

/// Parse an Etterna ReplaysV2 replay file. Any invalid lines are skipped
///
/// This function is fairly heavily optimized, due to usage in etterna-graph.
///
/// If you pass `false` for the `exact` parameter, a lossy float parsing function will be used,
/// which gains performance at the expense of accuracy.
pub fn parse_replay_v2_fast(bytes: &[u8], exact: bool) -> ReplayV2Fast {
	let parse_float: fn(&[u8]) -> Result<f32, _> = if exact {
		lexical_core::parse
	} else {
		lexical_core::parse_lossy
	};

	let approx_max_num_lines = bytes.len() / 16; // 16 is a pretty good approximation

	let mut notes = Vec::with_capacity(approx_max_num_lines);
	let mut num_mine_hits = 0;
	let mut num_hold_drops = 0;
	for line in crate::util::split_newlines(&bytes, 5) {
		if line.len() == 0 {
			continue;
		}

		if line[0usize] == b'H' {
			num_hold_drops += 1;
			continue;
		}

		let mut token_iter = line.splitn(3, |&c| c == b' ');

		let tick = token_iter.next().expect("Missing tick token");
		let tick: u32 = match btoi::btou(tick) {
			Ok(x) => x,
			Err(_) => continue,
		};

		let deviation = token_iter.next().expect("Missing deviation token");
		let deviation = if deviation.starts_with(b"1.0") {
			crate::Hit::Miss
		} else {
			let deviation = match parse_float(deviation) {
				Ok(x) => x,
				Err(_) => continue,
			};
			crate::Hit::Hit { deviation }
		};

		// remainder has the rest of the string in one slice, without any whitespace info or such.
		// luckily we know the points of interest's exact positions, so we can just directly index
		// into the remainder string to get what we need
		let remainder = token_iter.next().expect("Missing column token");
		let column: u8 = remainder[0] - b'0';
		let note_type: u8 = if remainder.len() >= 3 {
			remainder[2] - b'0'
		} else {
			1
		};

		// We only want tap notes and hold heads
		match note_type {
			1 | 2 => {
				// taps and hold heads
				notes.push(ReplayV2Note {
					tick,
					hit: deviation,
					column,
				});
			}
			4 => num_mine_hits += 1, // mines only appear in replay file if they were hit
			5 | 7 => {}              // lifts and fakes
			other => eprintln!("Warning: unexpected note type in replay file: {}", other),
		}
	}

	ReplayV2Fast {
		notes,
		num_mine_hits,
		num_hold_drops,
	}
}
