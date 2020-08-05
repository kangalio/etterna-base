use std::convert::{TryFrom, TryInto};

/// Skillset information. Used for chart specific difficulty, i.e. MSD and SSR
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChartSkillsets {
	pub stream: f32,
	pub jumpstream: f32,
	pub handstream: f32,
	pub stamina: f32,
	pub jackspeed: f32,
	pub chordjack: f32,
	pub technical: f32,
}
crate::impl_get8!(ChartSkillsets, f32, a, a.overall(), a.overall_pre_070());

impl ChartSkillsets {
	/// Return the overall skillset, as derived from the 7 individual skillsets
	pub fn overall(&self) -> f32 {
		let aggregated_skillsets = crate::rating_calc::calculate_score_overall(&[
			self.stream,
			self.jumpstream,
			self.handstream,
			self.stamina,
			self.jackspeed,
			self.chordjack,
			self.technical,
		]);
		let max_skillset = self.stream
			.max(self.jumpstream)
			.max(self.handstream)
			.max(self.stamina)
			.max(self.jackspeed)
			.max(self.chordjack)
			.max(self.technical);
		
		aggregated_skillsets.max(max_skillset)
	}

	/// Return the overall skillset with the pre-0.70 formula, as derived from the 7 individual
	/// skillsets
	pub fn overall_pre_070(&self) -> f32 {
		self.stream
			.max(self.jumpstream)
			.max(self.handstream)
			.max(self.stamina)
			.max(self.jackspeed)
			.max(self.chordjack)
			.max(self.technical)
	}
}

/// Skillset information. Used for player ratings
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserSkillsets {
	pub stream: f32,
	pub jumpstream: f32,
	pub handstream: f32,
	pub stamina: f32,
	pub jackspeed: f32,
	pub chordjack: f32,
	pub technical: f32,
}
crate::impl_get8!(UserSkillsets, f32, a, a.overall(), a.overall_pre_070());

impl UserSkillsets {
	/// Return the overall skillset, as derived from the 7 individual skillsets
	pub fn overall(&self) -> f32 {
		crate::rating_calc::calculate_player_overall(&[
			self.stream,
			self.jumpstream,
			self.handstream,
			self.stamina,
			self.jackspeed,
			self.chordjack,
			self.technical,
		])
	}

	/// Return the overall skillset with the pre-0.70 formula, as derived from the 7 individual
	/// skillsets
	pub fn overall_pre_070(&self) -> f32 {
		let sum = self.stream
			+ self.jumpstream
			+ self.handstream
			+ self.stamina
			+ self.jackspeed
			+ self.chordjack
			+ self.technical;
		sum / 7.0
	}
}

/// Skillsets enum, excluding overall
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Skillset7 {
	Stream,
	Jumpstream,
	Handstream,
	Stamina,
	Jackspeed,
	Chordjack,
	Technical,
}

impl Skillset7 {
	/// Same as [`Skillset8::from_user_input`]
	pub fn from_user_input(input: &str) -> Option<Self> {
		match Skillset8::from_user_input(input) {
			Some(skillset) => skillset.try_into().ok(),
			None => None,
		}
	}

	/// Get a list of all skillsets
	pub fn list() -> &'static [Self] {
		&[Self::Stream, Self::Jumpstream, Self::Handstream, Self::Stamina, Self::Jackspeed,
			Self::Chordjack, Self::Technical]
	}

	/// Iterate all skillsets
	pub fn iter() -> impl Iterator<Item=Self> {
		Self::list().iter().copied()
	}

	pub fn into_skillset8(self) -> Skillset8 {
		match self {
			Self::Stream => Skillset8::Stream,
			Self::Jumpstream => Skillset8::Jumpstream,
			Self::Handstream => Skillset8::Handstream,
			Self::Stamina => Skillset8::Stamina,
			Self::Jackspeed => Skillset8::Jackspeed,
			Self::Chordjack => Skillset8::Chordjack,
			Self::Technical => Skillset8::Technical,
		}
	}
}

/// Skillsets enum, including overall
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Skillset8 {
	Overall,
	Stream,
	Jumpstream,
	Handstream,
	Stamina,
	Jackspeed,
	Chordjack,
	Technical,
}

impl Skillset8 {
	/// Converts user input into a skillset variant, case-insensitively. Most community-accepted
	/// spellings of the skillsets are recognized.
	/// 
	/// Returns `None` If the given user input can't be parsed.
	/// 
	/// # Example
	/// ```rust
	/// # use etterna_base::Skillset8;
	/// assert_eq!(Some(Skillset8::Jumpstream), Skillset8::from_user_input("js"));
	/// assert_eq!(Some(Skillset8::Jackspeed), Skillset8::from_user_input("Jacks"));
	/// assert_eq!(Some(Skillset8::Jackspeed), Skillset8::from_user_input("JACKSPEED"));
	/// assert_eq!(None, Skillset8::from_user_input("handstreams"));
	/// ```
	pub fn from_user_input(input: &str) -> Option<Self> {
		match &input.to_lowercase() as &str {
			"overall" => Some(Self::Overall),
			"stream" => Some(Self::Stream),
			"js" | "jumpstream" => Some(Self::Jumpstream),
			"hs" | "handstream" => Some(Self::Handstream),
			"stam" | "stamina" => Some(Self::Stamina),
			"jack" | "jacks" | "jackspeed" => Some(Self::Jackspeed),
			"cj" | "chordjack" | "chordjacks" => Some(Self::Chordjack),
			"tech" | "technical" => Some(Self::Technical),
			_ => None,
		}
	}

	/// Get a list of all skillsets
	pub fn list() -> &'static [Self] {
		&[Self::Overall, Self::Stream, Self::Jumpstream, Self::Handstream, Self::Stamina,
			Self::Jackspeed, Self::Chordjack, Self::Technical]
	}

	/// Iterate all skillsets
	pub fn iter() -> impl Iterator<Item=Self> {
		Self::list().iter().copied()
	}

	pub fn into_skillset7(self) -> Option<Skillset7> {
		match self {
			Self::Overall => None,
			Self::Stream => Some(Skillset7::Stream),
			Self::Jumpstream => Some(Skillset7::Jumpstream),
			Self::Handstream => Some(Skillset7::Handstream),
			Self::Stamina => Some(Skillset7::Stamina),
			Self::Jackspeed => Some(Skillset7::Jackspeed),
			Self::Chordjack => Some(Skillset7::Chordjack),
			Self::Technical => Some(Skillset7::Technical),
		}
	}
}

impl TryFrom<Skillset8> for Skillset7 {
	type Error = ();

	fn try_from(ss: Skillset8) -> Result<Skillset7, ()> {
		ss.into_skillset7().ok_or(())
	}
}

impl std::convert::From<Skillset7> for Skillset8 {
	fn from(ss: Skillset7) -> Skillset8 {
		ss.into_skillset8()
	}
}

impl std::fmt::Display for Skillset7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for Skillset8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}