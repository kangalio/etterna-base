/// Returns the width in bytes of the first character in the string
/// 
/// Panics if the string is empty
fn first_char_width(string: &str) -> usize {
	for i in 1..10 { // dunno how far I need to go in
		if string.is_char_boundary(i) {
			return i;
		}
	}
	panic!("Can't determine first character's byte width in an empty string!")
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum CharToLane {
	Some(u32),
	Invalid,
	Space,
}

impl CharToLane {
	pub fn as_some(self) -> Option<u32> {
		match self {
			Self::Some(lane) => Some(lane),
			_ => None,
		}
	}
}

/// Convert a character in a pattern to a lane number. Works with numbers as well as LDUR.
fn char_to_lane(c: u8) -> CharToLane {
	match c.to_ascii_lowercase() {
		b'0' => CharToLane::Space,
		b'1'..=b'9' => CharToLane::Some((c - b'1') as u32),
		b'l' => CharToLane::Some(0),
		b'd' => CharToLane::Some(1),
		b'u' => CharToLane::Some(2),
		b'r' => CharToLane::Some(3),
		_ => CharToLane::Invalid,
	}
}

pub struct Pattern {
	/// Each row is a vector of lane numbers. For example a plain jumptrill would be
	/// `vec![vec![0, 1], vec![2, 3], vec![0, 1], vec![2, 3]...]`
	pub rows: Vec<Vec<u32>>,
}

impl Pattern {
	/// Guesses the keymode (e.g. 4k/5k/6k/...) by adding 1 to the rightmost lane. The number is
	/// clamped to a minimum of 4k - there is no such thing as 3k, 2k, 1k.
	/// 
	/// Returns None if the pattern is empty.
	///
	/// Note that this function returns only a _guess_. Nobody knows if [12][34] was intended as a
	/// 4k pattern, or a 5k, 6k, 7k...
	/// 
	/// ```rust
	/// # use etterna_base::Pattern;
	/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// assert_eq!(Pattern::parse_taps("1234")?.keymode(), Some(4));
	/// assert_eq!(Pattern::parse_taps("123")?.keymode(), Some(4));
	/// assert_eq!(Pattern::parse_taps("9")?.keymode(), Some(9));
	/// assert_eq!(Pattern::parse_taps("")?.keymode(), None);
	/// # Ok(()) }
	/// ```
	pub fn keymode(&self) -> Option<u32> {
		let keymode = 1 + self.rows.iter().flatten().max()?;

		// clamp to a minimum of 4 because even if the pattern is `2323`, it's still 4k
		let keymode = keymode.max(4);

		Some(keymode)
	}

	/// Parse a pattern from the format as it has established itself in the Etterna community.
	/// 
	/// The pattern syntax doesn't support mines, holds, rolls, lifts.
	/// 
	/// Gaps can be represented as `0` or `[]` (this extension is not widely established in the
	/// community)
	/// 
	/// This parser is super lenient. Any invalid characters are simply skipped over. Unterminated
	/// brackets are ignored too.
	/// 
	/// Examples:
	/// - `1234` for a roll
	/// - `[12][34][12][34]` for a jumptrill
	/// - `33303330333` for a jack with gaps on the right index finger
	/// 
	/// ```rust
	/// # use etterna_base::Pattern;
	/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// assert_eq!(
	/// 	Pattern::parse_taps("[1234]04"),
	/// 	Pattern { rows: vec![vec![0, 1, 2, 3], vec![], vec![3]] },
	/// );
	/// # Ok(()) }
	/// ```
	pub fn parse_taps(mut string: &str) -> Self {
		let mut rows = Vec::new();

		// this parser works by 'popping' characters off the start of the string until the string is empty

		while !string.is_empty() {
			// if the next char is a '[', find the matching ']', read all numbers inbetween, put them into a
			// vector, and finally add that vector to the `rows`
			// if the next char is _not_ a '[' and it's a valid number, push a new row with the an arrow in
			// the lane specified by the number
			if let (true, Some(end)) = (string.starts_with('['), string.find(']')) {
				rows.push(string[1..end].bytes()
					.filter_map(|c| char_to_lane(c).as_some())
					.collect::<Vec<_>>());
		
				string = &string[end+1..];
			} else {
				match char_to_lane(string.as_bytes()[0]) {
					CharToLane::Some(lane) => rows.push(vec![lane]),
					CharToLane::Space => rows.push(vec![]),
					CharToLane::Invalid => {},
				}
				
				string = &string[first_char_width(string)..];
			}
		}

		Pattern { rows }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// I think I'm making useless tests again. I probably don't even need tests for these miniscule
	// functions, in fact they're probably gonna change so often that these tests are gonna be out-
	// dated all the time..... eh whatever, the code is written, too late now.
	
	#[test]
	fn test_char_to_lane() {
		assert_eq!(char_to_lane(b'5'), CharToLane::Some(4));
		assert_eq!(char_to_lane(b'l'), CharToLane::Some(0));
		assert_eq!(char_to_lane(b'L'), CharToLane::Some(0));
		assert_eq!(char_to_lane(b'c'), CharToLane::Invalid);
		assert_eq!(char_to_lane(b'0'), CharToLane::Space);
	}

	#[test]
	fn test_first_char_width() {
		assert_eq!(first_char_width("a"), 1);
		assert_eq!(first_char_width("ä"), 2);
		assert_eq!(first_char_width("🔎"), 4);
	}

	#[test]
	#[should_panic]
	fn test_first_char_width_panic() {
		first_char_width("");
	}
}