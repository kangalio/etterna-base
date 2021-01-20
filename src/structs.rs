/// Chart difficulty enum
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Difficulty {
	Beginner,
	Easy,
	Medium,
	Hard,
	Challenge,
	Edit,
}

impl Difficulty {
	/// Parses a short difficulty string as found on the Etterna evaluation screen: BG, IN...
	///
	/// This function is case insensitive
	pub fn from_short_string(string: &str) -> Option<Self> {
		match string.to_ascii_uppercase().as_str() {
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
	/// This function is case insensitive
	pub fn from_long_string(string: &str) -> Option<Self> {
		match string.to_ascii_lowercase().as_str() {
			"beginner" | "novice" => Some(Self::Beginner),
			"easy" => Some(Self::Easy),
			"medium" | "normal" => Some(Self::Medium),
			"hard" => Some(Self::Hard),
			"challenge" | "expert" | "insane" => Some(Self::Challenge),
			"edit" => Some(Self::Edit),
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct DifficultyParseError;
impl std::fmt::Display for DifficultyParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "invalid difficulty")
	}
}
impl std::error::Error for DifficultyParseError {}

impl std::str::FromStr for Difficulty {
	type Err = DifficultyParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::from_long_string(s).ok_or(DifficultyParseError)
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

impl From<FullJudgements> for TapJudgements {
	fn from(judgements: FullJudgements) -> Self {
		Self {
			marvelouses: judgements.marvelouses,
			perfects: judgements.perfects,
			greats: judgements.greats,
			goods: judgements.goods,
			bads: judgements.bads,
			misses: judgements.misses,
		}
	}
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

/// Wifescore struct. Guaranteed to be a valid value, i.e. <= 100% and not NaN (may be negative
/// infinity though)
#[derive(PartialEq, PartialOrd, Default, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Wifescore {
	proportion: f32,
}

impl Wifescore {
	pub const D_THRESHOLD: Self = Self {
		proportion: f32::NEG_INFINITY,
	};
	pub const C_THRESHOLD: Self = Self { proportion: 0.60 };
	pub const B_THRESHOLD: Self = Self { proportion: 0.70 };
	pub const A_THRESHOLD: Self = Self { proportion: 0.80 };
	pub const AA_THRESHOLD: Self = Self { proportion: 0.93 };
	pub const AAA_THRESHOLD: Self = Self { proportion: 0.997 };
	pub const AAAA_THRESHOLD: Self = Self {
		proportion: 0.99955,
	};
	pub const AAAAA_THRESHOLD: Self = Self {
		proportion: 0.99996,
	};

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
		if proportion.is_nan() || proportion > 1.0 {
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

#[allow(clippy::derive_ord_xor_partial_ord)] // the reasoning doesn't apply here
impl Ord for Wifescore {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		other
			.partial_cmp(other)
			.expect("Can't happen; this wrapper guarantees non-NaN")
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

	fn mean_deviation(&self) -> f32 {
		let mut num_deviations = 0;
		let mut deviations_sum = 0.0;

		for hit in self.iter_hits() {
			if let crate::Hit::Hit { deviation } = hit {
				num_deviations += 1;
				deviations_sum += deviation;
			}
		}

		deviations_sum / num_deviations as f32
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

impl TapJudgement {
	pub fn color(self) -> (u8, u8, u8) {
		match self {
			Self::Marvelous => (0x99, 0xCC, 0xFF),
			Self::Perfect => (0xF2, 0xCB, 0x30),
			Self::Great => (0x14, 0xCC, 0x8F),
			Self::Good => (0x1A, 0xB2, 0xFF),
			Self::Bad => (0xFF, 0x1A, 0xB3),
			Self::Miss => (0xCC, 0x29, 0x29),
		}
	}
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

	pub fn is_within_window(&self, window: f32) -> bool {
		assert!(window >= 0.0);

		match *self {
			Self::Hit { deviation } => deviation.abs() < window,
			Self::Miss => false,
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

	/// Whether this hit is considered a miss. Note the distinction to [`Self::was_missed`]! This
	/// can return true even if the user _did_ hit the note: A deviation of 200ms is a late bad on
	/// J1 but a "miss" on J4.
	///
	/// If you want to check whether the note was truly **missed**, use `if hit == Hit::Miss` or
	/// `if let Hit::Miss = hit` instead.
	///
	/// ```rust
	/// # use etterna_base::{Hit, J1, J4};
	/// assert!(Hit::Hit { deviation: -0.02 }.is_considered_miss(J4) == false);
	/// assert!(Hit::Miss.is_considered_miss(J4) == true);
	/// assert!(Hit::Hit { deviation: 0.20 }.is_considered_miss(J1) == false);
	/// assert!(Hit::Hit { deviation: 0.20 }.is_considered_miss(J4) == true);
	/// ```
	pub fn is_considered_miss(&self, judge: &crate::Judge) -> bool {
		match *self {
			Self::Hit { deviation } => judge.is_miss(deviation),
			Self::Miss => true,
		}
	}

	/// Whether the note has not been hit by the player. Note the distincion to
	/// [`Self::is_considered_miss`]! Even a, say, 250ms late hit from J1 would return `false`
	/// here.
	///
	/// ```rust
	/// # use etterna_base::{Hit, J1, J4};
	/// assert!(Hit::Hit { deviation: -0.02 }.was_missed() == false);
	/// assert!(Hit::Miss.was_missed() == true);
	/// assert!(Hit::Hit { deviation: 0.20 }.was_missed() == false);
	/// assert!(Hit::Hit { deviation: 0.20 }.was_missed() == false);
	/// ```
	pub fn was_missed(&self) -> bool {
		match *self {
			Self::Hit { .. } => false,
			Self::Miss => true,
		}
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Hash)]
pub struct NoteRow {
	// least significant bit is leftmost finger
	bits: u32,
}

impl NoteRow {
	/// Instantiates from a bitset stored in an integer, where the least significant bit corresponds
	/// to the leftmost lane.
	pub fn from_bits(bits: u32) -> Self {
		Self { bits }
	}

	/// Returns the notes as a bitset, where the least significant bit corresponds to the leftmost
	/// lane
	pub fn bits(self) -> u32 {
		self.bits
	}

	/// Returns whether there is a tap at the given index, where 0 is the leftmost lane.
	pub fn tap_at(self, index: u32) -> bool {
		(self.bits & (1 << index)) > 0
	}

	/// Returns the number of notes that this row spans
	///
	/// ```rust
	/// # use etterna_base::NoteRow;
	/// assert_eq!(NoteRow::from_bits(0b10101).width(), 5);
	/// assert_eq!(NoteRow::from_bits(0b0011).width(), 2); // be careful!
	/// ```
	pub fn width(self) -> u32 {
		let bit_width = std::mem::size_of_val(&self.bits) as u32 * 8;
		bit_width - self.bits.leading_zeros()
	}
}

/*impl NoteRow {
	fn bit_width(self) -> u32 {
		std::mem::size_of_val(self.bits) * 8
	}

	pub fn new() -> Self {

	}

	/// Instantiates from a bitset stored in an integer, where the least significant bit corresponds
	/// to the leftmost lane
	///
	/// ```rust
	/// # use crate::NoteRow;
	/// assert_eq!(&NoteRow::from_bits_lsb_left(0b1110).format('x'), " xxx");
	/// ```
	pub fn from_bits_lsb_left(bits: u32) -> Self {
		Self { bits }
	}

	/// Instantiates from a bitset stored in an integer, where the least significant bit corresponds
	/// to the rightmost lane
	///
	/// ```rust
	/// # use crate::NoteRow;
	/// assert_eq!(&NoteRow::from_bits_lsb_right(0b1110).format('x'), "xxx ");
	/// assert_ne!(&NoteRow::from_bits_lsb_right(0b0111).format('x'), "xxx"); // be careful!
	/// ```
	pub fn from_bits_lsb_right(bits: u32) -> Self {
		Self { bits }.mirror()
	}

	/// Returns a bit representation of this note row where the least significant bit corresponds
	/// to the leftmost lane
	pub fn bits_lsb_left(self) -> u32 {
		self.bits
	}

	/// Returns a bit representation of this note row where the least significant bit corresponds
	/// to the rightmost lane
	pub fn bits_lsb_right(self) -> u32 {
		self.mirror().bits
	}

	/// Returns the number of notes in this row
	pub fn num_notes(self) -> u32 {
		self.bits.count_ones()
	}

	/// Mirror the notes, so that any notes on the left end up on the right and vice-versa
	///
	/// ```rust
	/// # use crate::NoteRow;
	/// assert_eq!(NoteRow::from_lsb_right(0b110).mirror(), NoteRow::from_lsb_right(0b011));
	/// assert_eq!(NoteRow::from_lsb_right(0b11001).mirror(), NoteRow::from_lsb_right(0b10011));
	/// ```
	pub fn mirror(self) -> Self {
		Self { bits: self.bits.reverse_bits() >> self.bits.leading_zeros() }
	}
}*/

impl std::fmt::Debug for NoteRow {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for i in 0..self.width() {
			write!(f, "{}", if self.tap_at(i) { "x" } else { " " })?;
		}
		Ok(())
	}
}

/// No guaranteess of any sorts about ordering or contents in general
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NoteAndHitSeconds {
	pub note_seconds: Vec<f32>,
	pub hit_seconds: Vec<f32>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScrollDirection {
	Upscroll,
	Downscroll,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Snap {
	_4th,
	_8th,
	_12th,
	_16th,
	_24th,
	_32th,
	_48th,
	_64th,
	_192th,
}

impl Snap {
	pub fn from_row(row: u32) -> Self {
		if row % (192 / 4) == 0 {
			Self::_4th
		} else if row % (192 / 8) == 0 {
			Self::_8th
		} else if row % (192 / 12) == 0 {
			Self::_12th
		} else if row % (192 / 16) == 0 {
			Self::_16th
		} else if row % (192 / 24) == 0 {
			Self::_24th
		} else if row % (192 / 32) == 0 {
			Self::_32th
		} else if row % (192 / 48) == 0 {
			Self::_48th
		} else if row % (192 / 64) == 0 {
			Self::_64th
		} else {
			Self::_192th
		}
	}
}
