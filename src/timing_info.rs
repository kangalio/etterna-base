use thiserror::Error;

#[derive(Debug)]
pub struct BpmChange {
	beat: f64,
	bpm: f64,
}

#[derive(Debug)]
pub struct TimingInfo {
	first_bpm: f64,
	// Must be chronologically ordered!
	changes: Vec<BpmChange>,
}

#[derive(Debug, Error)]
pub enum SmBpmStringParseError {
	#[error("No equals sign in bpms entry")]
	MissingEquals,
	#[error("Could not parse the beat into a float: {0:?}")]
	InvalidBeatString(lexical_core::Error),
	#[error("Could not parse the bpm into a float: {0:?}")]
	InvalidBpmString(lexical_core::Error),
	#[error("First bpm change is not at beat=0, it's at beat={beat_instead:?}")]
	FirstBpmChangeNotZero { beat_instead: f64 },
	#[error("A bpm or beat was NaN")]
	EncounteredNan,
}

impl TimingInfo {
	pub fn from_sm_bpm_string(string: &[u8]) -> Result<Self, SmBpmStringParseError> {
		// rough capacity approximation
		let mut changes = Vec::with_capacity(string.len() / 13);
		
		for pair in string.split(|&c| c == b',') {
			let equal_sign_index = pair.iter().position(|&c| c == b'=')
				.ok_or(SmBpmStringParseError::MissingEquals)?;
			let beat: f64 = lexical_core::parse_lossy(crate::util::trim_bstr(&pair[..equal_sign_index]))
				.map_err(SmBpmStringParseError::InvalidBeatString)?;
			let bpm: f64 = lexical_core::parse_lossy(crate::util::trim_bstr(&pair[equal_sign_index+1..]))
				.map_err(SmBpmStringParseError::InvalidBpmString)?;

			if beat.is_nan() || bpm.is_nan() {
				return Err(SmBpmStringParseError::EncounteredNan);
			}

			changes.push(BpmChange { beat, bpm });
		}
		
		// UNWRAP: We checked in the loop above that all these numbers are non-NaN
		changes.sort_by(|a, b| a.beat.partial_cmp(&b.beat).unwrap());
		
		if changes[0].beat != 0.0 {
			return Err(SmBpmStringParseError::FirstBpmChangeNotZero { beat_instead: changes[0].beat });
		}
		let first_bpm = changes[0].bpm;
		changes.remove(0); // remove first entry (0.0=xxx)
		
		Ok(TimingInfo { changes, first_bpm })
	}

	/// Input slice must be sorted!
	pub fn ticks_to_seconds(&self, ticks: &[u32]) -> Vec<f32> {
		assert!(crate::util::is_sorted(ticks));
		
		let mut cursor_beat: f64 = 0.0;
		let mut cursor_second: f64 = 0.0;
		let mut beat_time = 60.0 / self.first_bpm;
		
		// if a tick lies exactly on the boundary, if will _not_ be processed
		let mut ticks_i = 0;
		let mut seconds_vec = Vec::with_capacity(ticks.len());
		let mut convert_ticks_up_to = |beat: f64, cursor_second: f64, cursor_beat: f64, beat_time: f64| {
			while ticks_i < ticks.len() && ticks[ticks_i] as f64 / 48.0 < beat {
				let beat = ticks[ticks_i] as f64 / 48.0;
				let second = cursor_second + (beat - cursor_beat) * beat_time;
				seconds_vec.push(second as f32);
				
				ticks_i += 1;
			}
		};
		
		for BpmChange { beat: change_beat, bpm: change_bpm } in &self.changes {
			convert_ticks_up_to(*change_beat, cursor_second, cursor_beat, beat_time);
			
			cursor_second += beat_time * (change_beat - cursor_beat);
			cursor_beat = *change_beat;
			beat_time = 60.0 / change_bpm;
		}
		
		// process all remaining ticks (i.e. all ticks coming after the last bpm change
		convert_ticks_up_to(f64::INFINITY, cursor_second, cursor_beat, beat_time);
		
		assert!(ticks.len() == seconds_vec.len()); // If this panics, the above code is wrong
		
		seconds_vec
	}
}