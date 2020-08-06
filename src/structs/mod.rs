//! A collection of common Etterna structs and related functions

mod skillsets;
pub use skillsets::*;

use thiserror::Error;

/// Chart difficulty enum
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Difficulty {
	Beginner, Easy, Medium, Hard, Challenge, Edit
}

impl Difficulty {
	/// Parses a short difficulty string as found on the Etterna evaluation screen: BG, IN...
	///
	/// The string must be given in uppercase letters
	pub fn from_short_string(string: &str) -> Option<Self> {
		match string {
			"BG" => Some(Self::Beginner),
			"EZ" => Some(Self::Easy),
			"NM" => Some(Self::Medium),
			"HD" => Some(Self::Hard),
			"IN" => Some(Self::Challenge),
			"ED" => Some(Self::Edit),
			_ => None,
		}
	}

	/// Parse a long difficulty string. Some difficulties has multiple spellings; for example
	/// "Challenge", "Expert" and "Insane".
	/// 
	/// Strings must be given with first letter uppercase and the rest lowercase
	pub fn from_long_string(string: &str) -> Option<Self> {
		match string {
			"Beginner" | "Novice" => Some(Self::Beginner),
			"Easy" => Some(Self::Easy),
			"Medium" | "Normal" => Some(Self::Medium),
			"Hard" => Some(Self::Hard),
			"Challenge" | "Expert" | "Insane" => Some(Self::Challenge),
			"Edit" => Some(Self::Edit),
			_ => None,
		}
	}

	/// Generate a short difficulty string as found on the Etterna evaluation screen.
	pub fn to_short_string(self) -> &'static str {
		match self {
			Self::Beginner => "BG",
			Self::Easy => "EZ",
			Self::Medium => "NM",
			Self::Hard => "HD",
			Self::Challenge => "IN",
			Self::Edit => "ED",
		}
	}
}

/// Judgement data, including mines and holds
#[derive(Debug, Eq, PartialEq, Clone, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FullJudgements {
	pub marvelouses: u32,
	pub perfects: u32,
	pub greats: u32,
	pub goods: u32,
	pub bads: u32,
	pub misses: u32,
	pub hit_mines: u32,
	pub held_holds: u32,
	pub let_go_holds: u32,
	pub missed_holds: u32,
}

/// Judgement data, only the basic tap judgements
#[derive(Debug, Eq, PartialEq, Clone, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TapJudgements {
	pub marvelouses: u32,
	pub perfects: u32,
	pub greats: u32,
	pub goods: u32,
	pub bads: u32,
	pub misses: u32,
}

/// Type of a note
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NoteType {
	Tap,
	HoldHead,
	HoldTail,
	Mine,
	Lift,
	Keysound,
	Fake,
}

/// Represents an Etterna chart rate (music speed).
/// 
/// As in Etterna, this value can only be a multiple of 0.05. The value can't be negative, nor NaN
/// or infinity.
/// 
/// When printed, a [`Rate`] is formatted as usual in Etterna; two floating point digits and an `x`
/// at the end: `0.85x, 1.00x, 2.40x`
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rate {
	// this value is 20x the real rate, e.g. `1.15x` would be 23
	x20: u32,
}

impl Rate {
	/// Rounds to the nearest valid rate.
	/// 
	/// Returns None if the given value is negative or too large
	pub fn from_f32(r: f32) -> Option<Self> {
		// Some(Self { x20: (r * 20.0).round().try_into().ok()? })
		if r < 0.0 || r > u32::MAX as f32 {
			None
		} else {
			Some(Self { x20: (r * 20.0).round() as u32 })
		}
	}

	/// Parses a string into a rate. The string needs to be in the format `\d+\.\d+[05]?`
	/// 
	/// Returns None if parsing failed
	pub fn from_string(string: &str) -> Option<Self> {
		// not the most efficient but /shrug
		Self::from_f32(string.parse().ok()?)
	}

	/// Create a new rate from a value that is equal to the real rate multiplied by 20.
	/// 
	/// Due to the fact that Etterna ratings are always multiples of 0.05, every rate can be
	/// precicely represented precisely with a whole number when multiplied by 20.
	pub fn from_x20(x20: u32) -> Self {
		Self { x20 }
	}

	/// Returns an f32 representation of this rate.
	/// 
	/// ```rust
	/// # use etterna_base::structs::Rate;
	/// assert_eq!(Rate::from_string("1.40").unwrap().as_f32(), 1.4);
	/// ```
	pub fn as_f32(self) -> f32 {
		self.x20 as f32 / 20.0
	}
}

impl std::fmt::Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}x", self.x20 as f32 / 20.0)
    }
}

impl std::fmt::Debug for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} / 20.0)x", self.x20)
    }
}

impl Default for Rate {
    fn default() -> Self {
        Self::from_x20(20)
    }
}

/// Wifescore struct. Guaranteed to be a valid value, i.e. not infinity and not NaN
#[derive(PartialEq, PartialOrd, Default, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Wifescore {
	proportion: f32,
}

impl Wifescore {
	/// Makes a Wifescore from a value, assumed to be scaled to a max of 100
	/// 
	/// Returns None if the percentage is over 100%, or if it is infinite or NaN
	pub fn from_percent(percent: f32) -> Option<Self> {
		Self::from_proportion(percent / 100.0)
	}

	/// Makes a Wifescore from a value, assumed to be scaled to a max of 1
	/// 
	/// Returns None if the proportion is over 1.0 (100%), or if it is infinite or NaN
	pub fn from_proportion(proportion: f32) -> Option<Self> {
		if proportion.is_infinite() || proportion.is_nan() || proportion > 1.0 {
			None
		} else {
			Some(Self { proportion })
		}
	}

	/// Returns the wifescore, scaled to a max of 100
	pub fn as_percent(self) -> f32 {
		self.proportion * 100.0
	}

	/// Returns the wifescore, scaled to a max of 1
	pub fn as_proportion(self) -> f32 {
		self.proportion
	}
}

impl std::fmt::Display for Wifescore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.as_percent())
    }
}

impl Ord for Wifescore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		other.partial_cmp(other).expect("Can't happen; this wrapper guarantees non-NaN")
    }
}

// This can't be a derive for whatever reason /shrug
impl Eq for Wifescore {}

// we need this wrapper because REDACTED
macro_rules! doc_comment {
	($comment:expr, $($stuff:tt)*) => { #[doc = $comment] $($stuff)* };
}

// Implementation for both chartkey and scorekey (and potentially even songkey in the future? maybe
// once I figure out what the hell songkey even is)
macro_rules! etterna_data_key {
	($name:ident, $name_lowercase:ident, $initial_letter:expr) => (
		// TODO: maybe it's a good idea to represent this as [u8; 20] instead? not sure
		doc_comment! { concat!("Represents an Etterna ", stringify!($name_lowercase)),
			#[derive(Debug, Clone, Eq, PartialEq, Hash, /* NOT Default, it would produce an invalid state! */)]
			#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
			pub struct $name(String);
		}

		impl $name {
			/// Check if the given string represents a valid key
			pub fn is_valid(key: &str) -> bool {
				let initial_letter: char = $initial_letter;

				key.len() == 41
					&& key.starts_with(initial_letter)
					&& !key[1..].contains(|c| (c < '0' || c > '9') && (c < 'a' || c > 'f'))
			}

			/// Create a new key from the given string, or None if the string is invalid.
			pub fn new(key: String) -> Option<Self> {
				if Self::is_valid(&key) {
					Some(Self(key))
				} else {
					None
				}
			}

			/// Get a reference to the underlying string
			pub fn as_str(&self) -> &str {
				&self.0
			}

			/// Convert this key into a `String`
			pub fn into_string(self) -> String {
				self.0
			}
		}

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", self.as_str())
			}
		}

		impl From<$name> for String {
			fn from(key: $name) -> String { key.into_string() }
		}

		impl AsRef<str> for $name {
			fn as_ref(&self) -> &str { self.as_str() }
		}

		impl std::convert::TryFrom<String> for $name {
			type Error = ();
			fn try_from(key: String) -> Result<Self, ()> { Self::new(key).ok_or(()) }
		}
	)
}

etterna_data_key!(Scorekey, scorekey, 'S');
etterna_data_key!(Chartkey, chartkey, 'X');

/// Represents a file size
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileSize {
	bytes: u64,
}

impl FileSize {
	/// Create a new file size from the given number of bytes
	pub fn from_bytes(bytes: u64) -> Self {
		Self { bytes }
	}

	/// Get the number of bytes
	pub fn bytes(self) -> u64 { self.bytes }

	/// Get the number of kilobytes, rounded down
	pub fn kb(self) -> u64 { self.bytes / 1_000 }

	/// Get the number of megabytes, rounded down
	pub fn mb(self) -> u64 { self.bytes / 1_000_000 }

	/// Get the number of gigabytes, rounded down
	pub fn gb(self) -> u64 { self.bytes / 1_000_000_000 }

	/// Get the number of terabytes, rounded down
	pub fn tb(self) -> u64 { self.bytes / 1_000_000_000_000 }
}

/// Error returned from `FileSize::from_str`
#[derive(Debug, Error)]
pub enum FileSizeParseError {
	#[error("Given string was empty")]
	EmptyString,
	#[error("Error while parsing the filesize number")]
	InvalidNumber(#[source] std::num::ParseFloatError),
	#[error("No KB/MB/... ending")]
	NoEnding,
	#[error("Unknown ending (the KB/MB/... thingy)")]
	UnexpectedEnding(String),
}

impl std::str::FromStr for FileSize {
	type Err = FileSizeParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut token_iter = s.split_whitespace();
		let number: f64 = token_iter.next().ok_or(FileSizeParseError::EmptyString)?
			.parse().map_err(FileSizeParseError::InvalidNumber)?;
		let ending = token_iter.next().ok_or(FileSizeParseError::NoEnding)?;

		let ending = ending.to_lowercase();
		let multiplier: u64 = match &ending as &str {
			"b"	  => 1,
			"kb"  => 1000,
			"kib" => 1024,
			"mb"  => 1000 * 1000,
			"mib" => 1024 * 1024,
			"gb"  => 1000 * 1000 * 1000,
			"gib" => 1024 * 1024 * 1024,
			"tb"  => 1000 * 1000 * 1000 * 1000,
			"tib" => 1024 * 1024 * 1024 * 1024,
			_ => return Err(FileSizeParseError::UnexpectedEnding(ending)),
		};

		Ok(Self::from_bytes((number * multiplier as f64) as u64))
	}
}

/// Global ranks in each skillset category
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserRank {
	pub overall: u32,
	pub stream: u32,
	pub jumpstream: u32,
	pub handstream: u32,
	pub stamina: u32,
	pub jackspeed: u32,
	pub chordjack: u32,
	pub technical: u32,
}

impl UserRank {
	crate::impl_get8!(u32, a, a.overall);
}