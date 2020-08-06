/// Contents of an Etterna ReplayV2 replay file. See [`parse_replay`] for more
pub struct ReplayFileData {
	pub notes: Vec<ReplayFileNote>,
	pub num_mine_hits: u32,
	pub num_hold_drops: u32,
}

/// Represents a single note in a [`ReplayFileData`]
pub struct ReplayFileNote {
	pub tick: u32,
	pub deviation: f32,
	pub column: u8,
}

fn parse_sm_float(string: &[u8]) -> Option<f32> {
	//~ let string = &string[..string.len()-1]; // cut off last digit to speed up float parsing # REMEMBER
	lexical_core::parse_lossy(string).ok()
	
	/*
	// For performance reasons, this assumes that the passed-in bytestring is in the format
	// -?[01]\.\d{6} (optionally a minus, then 0 or 1, a dot, and then 6 floating point digits. (This
	// function only parses 5 of those floating point digits though). Example string: "-0.010371"
	
	let is_negative = string[0] == b'-';
	let string = if is_negative { &string[1..] } else { string }; // Strip minus
	
	let mut digits_part: u32 = 0;
	digits_part += (string[6] - b'0') as u32 * 1;
	digits_part += (string[5] - b'0') as u32 * 10;
	digits_part += (string[4] - b'0') as u32 * 100;
	digits_part += (string[3] - b'0') as u32 * 1000;
	digits_part += (string[2] - b'0') as u32 * 10000;
	digits_part += (string[0] - b'0') as u32 * 100000;
	
	let mut number = digits_part as f32 / 100000 as f32;
	
	if is_negative {
		number = -number;
	}
	
	return Some(number);
	*/
}

/// Parse an Etterna ReplaysV2 replay file. Any invalid lines are skipped
/// 
/// This function is fairly heavily optimized, due to usage in etterna-graph.
pub fn parse_replay(bytes: &[u8]) -> ReplayFileData {
	let approx_max_num_lines = bytes.len() / 16; // 16 is a pretty good approximation	
	
	let mut notes = Vec::with_capacity(approx_max_num_lines);
	let mut num_mine_hits = 0;
	let mut num_hold_drops = 0;
	for line in crate::util::split_newlines(&bytes, 5) {
		if line.len() == 0 { continue }
		
		if line[0usize] == b'H' {
			num_hold_drops += 1;
			continue;
		}
		
		let mut token_iter = line.splitn(3, |&c| c == b' ');
		
		let tick = token_iter.next().expect("Missing tick token");
		let tick: u32 = match btoi::btou(tick) { Ok(x) => x, Err(_) => continue };
		let deviation = token_iter.next().expect("Missing tick token");
		let deviation = match parse_sm_float(deviation) { Some(x) => x, None => continue } as f32;
		// remainder has the rest of the string in one slice, without any whitespace info or such.
		// luckily we know the points of interest's exact positions, so we can just directly index
		// into the remainder string to get what we need
		let remainder = token_iter.next().expect("Missing tick token");
		let column: u8 = remainder[0] - b'0';
		let note_type: u8 = if remainder.len() >= 3 { remainder[2] - b'0' } else { 1 };
		
		// We only want tap notes and hold heads
		match note_type {
			1 | 2 => { // taps and hold heads
				notes.push(ReplayFileNote { tick, deviation, column });
			},
			4 => num_mine_hits += 1, // mines only appear in replay file if they were hit
			5 | 7 => {}, // lifts and fakes
			other => eprintln!("Warning: unexpected note type in replay file: {}", other),
		}
	}
	
	ReplayFileData { notes, num_mine_hits, num_hold_drops }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::assert_float_eq;
	
	#[test]
	fn test_sm_float_parsing() {
		assert_float_eq!(parse_sm_float(b"-0.018477").unwrap(), -0.018477;
				epsilon=0.00001);
		assert_float_eq!(parse_sm_float(b"1.000000").unwrap(), 1.000000;
				epsilon=0.00001);
		assert_float_eq!(parse_sm_float(b"0.919191").unwrap(), 0.919191;
				epsilon=0.00001);
	}
}