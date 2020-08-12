//! A collection of common Etterna structs and related functions

mod skillsets;
pub use skillsets::*;

mod file_size;
pub use file_size::*;

mod rate;
pub use rate::*;

mod timing_info;
pub use timing_info::*;

mod judge;
pub use judge::*;

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

impl std::ops::Index<crate::TapJudgement> for TapJudgements {
	type Output = u32;
	
    fn index(&self, index: crate::TapJudgement) -> &Self::Output {
        match index {
			crate::TapJudgement::Marvelous => &self.marvelouses,
			crate::TapJudgement::Perfect => &self.perfects,
			crate::TapJudgement::Great => &self.greats,
			crate::TapJudgement::Good => &self.goods,
			crate::TapJudgement::Bad => &self.bads,
			crate::TapJudgement::Miss => &self.misses,
		}
    }
}

impl std::ops::IndexMut<crate::TapJudgement> for TapJudgements {
    fn index_mut(&mut self, index: crate::TapJudgement) -> &mut Self::Output {
        match index {
			crate::TapJudgement::Marvelous => &mut self.marvelouses,
			crate::TapJudgement::Perfect => &mut self.perfects,
			crate::TapJudgement::Great => &mut self.greats,
			crate::TapJudgement::Good => &mut self.goods,
			crate::TapJudgement::Bad => &mut self.bads,
			crate::TapJudgement::Miss => &mut self.misses,
		}
    }
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
        write!(f, "{:.2}%", self.as_percent())
    }
}

impl Ord for Wifescore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		other.partial_cmp(other).expect("Can't happen; this wrapper guarantees non-NaN")
    }
}

// This can't be a derive for whatever reason /shrug
impl Eq for Wifescore {}

// we need this wrapper because <!'#]]]A~REDÅCTED~{#"$ ")=}
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

		impl std::str::FromStr for $name {
			type Err = ();
			fn from_str(key: &str) -> Result<Self, ()> {
				Self::new(key.to_owned()).ok_or(())
			}
		}
	)
}

etterna_data_key!(Scorekey, scorekey, 'S');
etterna_data_key!(Chartkey, chartkey, 'X');

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
	crate::impl_get_skillset!(u32, a, a.overall);
}

pub trait SimpleReplay {
	fn iter_hits(&self) -> Box<dyn '_ + Iterator<Item = crate::Hit>>;

	// TODO
	// fn rescore<W: crate::Wife>(&self) -> crate::Wifescore { todo!() }

	/// Finds the longest combo of notes evaluating to true in the given closure
	/// 
	/// The note deviations passed into the closure are always positive. In case of a miss, the
	/// closure call is skipped entirely and `false` is inserted.
	/// 
	/// # Example
	/// Find the longest marvelous combo:
	/// ```rust,ignore
	/// let longest_marvelous_combo = replay.longest_combo(|d| d.is_marv(etterna::J4));
	/// ```
	fn longest_combo(&self, hit_filter: impl FnMut(crate::Hit) -> bool) -> u32 {
		crate::util::longest_true_sequence(self.iter_hits().map(hit_filter))
	}

	/// Generate a [`crate::TapJudgements`] instance of this replay
	fn tap_judgements(&self, judge: &crate::Judge) -> crate::TapJudgements {
		let mut judgements = TapJudgements::default();
		for hit in self.iter_hits() {
			judgements[hit.classify(judge)] += 1;
		}
		judgements
	}
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TapJudgement {
	Marvelous,
	Perfect,
	Great,
	Good,
	Bad,
	Miss,
}

/// Represents a player hit of a single note
/// 
/// The deviation value is in seconds and may be negative
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Hit {
	Hit { deviation: f32 },
	Miss,
}

impl Hit {
	pub fn deviation(&self) -> Option<f32> {
		match *self {
			Self::Hit { deviation } => Some(deviation),
			Self::Miss => None,
		}
	}

	pub fn classify(&self, judge: &crate::Judge) -> TapJudgement {
		match *self {
			Self::Hit { deviation } => judge.classify(deviation),
			Self::Miss => TapJudgement::Miss,
		}
	}

	pub fn is_cb(&self, judge: &crate::Judge) -> bool {
		match *self {
			Self::Hit { deviation } => judge.is_cb(deviation),
			Self::Miss => false,
		}
	}

	pub fn is_marv(&self, judge: &crate::Judge) -> bool {
		match *self {
			Self::Hit { deviation } => judge.is_marv(deviation),
			Self::Miss => false,
		}
	}

	pub fn is_perf(&self, judge: &crate::Judge) -> bool {
		match *self {
			Self::Hit { deviation } => judge.is_perf(deviation),
			Self::Miss => false,
		}
	}

	pub fn is_great(&self, judge: &crate::Judge) -> bool {
		match *self {
			Self::Hit { deviation } => judge.is_great(deviation),
			Self::Miss => false,
		}
	}

	pub fn is_good(&self, judge: &crate::Judge) -> bool {
		match *self {
			Self::Hit { deviation } => judge.is_good(deviation),
			Self::Miss => false,
		}
	}

	pub fn is_bad(&self, judge: &crate::Judge) -> bool {
		match *self {
			Self::Hit { deviation } => judge.is_bad(deviation),
			Self::Miss => false,
		}
	}

	/// Whether this hit is considered a miss
	/// 
	/// ```rust
	/// # use etterna_base::{Hit, J1, J4};
	/// assert!(Hit::Hit { deviation: -0.02 }.is_miss(J4) == false);
	/// assert!(Hit::Hit { deviation: 0.20 }.is_miss(J4) == true);
	/// assert!(Hit::Hit { deviation: 0.20 }.is_miss(J1) == false);
	/// assert!(Hit::Miss.is_miss(J1) == true);
	/// ```
	pub fn is_miss(&self, judge: &crate::Judge) -> bool {
		match *self {
			Self::Hit { deviation } => judge.is_miss(deviation),
			Self::Miss => true,
		}
	}
}