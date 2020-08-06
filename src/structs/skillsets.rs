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

impl ChartSkillsets {
	crate::impl_get8!(f32, a, a.overall(), a.overall_pre_070());

	/// Return the overall skillset, as derived from the 7 individual skillsets
	/// 
	/// ```rust
	/// # use etterna_base::ChartSkillsets;
	/// // Fennec Fantasy - Friday Fahrenheit
	/// let msd = ChartSkillsets {
	/// 	stream: 22.31,
	/// 	jumpstream: 22.37,
	/// 	handstream: 18.99,
	/// 	stamina: 21.53,
	/// 	jackspeed: 14.12,
	/// 	chordjack: 15.85,
	/// 	technical: 21.47,
	/// };
	/// 
	/// let overall_difficulty = msd.overall();
	/// assert!((overall_difficulty - 22.99).abs() < 0.01);
	/// ```
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
	/// 
	/// ```rust
	/// # use etterna_base::ChartSkillsets;
	/// // Fennec Fantasy - Friday Fahrenheit
	/// let msd = ChartSkillsets {
	/// 	stream: 22.31,
	/// 	jumpstream: 22.37,
	/// 	handstream: 18.99,
	/// 	stamina: 21.53,
	/// 	jackspeed: 14.12,
	/// 	chordjack: 15.85,
	/// 	technical: 21.47,
	/// };
	/// 
	/// let overall_difficulty = msd.overall_pre_070();
	/// assert!((overall_difficulty - 22.37).abs() < 0.01);
	/// ```
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

impl UserSkillsets {
	crate::impl_get8!(f32, a, a.overall(), a.overall_pre_070());

	/// Return the overall skillset, as derived from the 7 individual skillsets
	/// 
	/// ```rust
	/// # use etterna_base::UserSkillsets;
	/// let player_rating = UserSkillsets {
	/// 	stream: 28.3815,
	/// 	jumpstream: 29.0849,
	/// 	handstream: 29.3894,
	/// 	stamina: 28.8329,
	/// 	jackspeed: 22.2704,
	/// 	chordjack: 27.2160,
	/// 	technical: 27.8669,
	/// };
	/// 
	/// let overall_rating = player_rating.overall();
	/// assert!((overall_rating - 28.7212).abs() < 0.01);
	/// ```
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
	/// 
	/// ```rust
	/// # use etterna_base::UserSkillsets;
	/// let player_rating = UserSkillsets {
	/// 	stream: 27.6060,
	/// 	jumpstream: 27.6567,
	/// 	handstream: 28.5327,
	/// 	stamina: 27.7139,
	/// 	jackspeed: 25.4858,
	/// 	chordjack: 27.8027,
	/// 	technical: 27.8662,
	/// };
	/// 
	/// let overall_rating = player_rating.overall_pre_070();
	/// assert!((overall_rating - 27.5234).abs() < 0.0001);
	/// ```
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
	/// 
	/// Useful for tasks that require operating on all skillsets individually
	/// 
	/// ```rust,no_run
	/// # use etterna_base::Skillset7;
	/// # let skillsets: etterna_base::UserSkillsets = unimplemented!();
	/// for &ss in Skillset7::list() {
	/// 	println!("{}: {}", ss, skillsets.get(ss));
	/// }
	/// ```
	pub fn list() -> &'static [Self] {
		&[Self::Stream, Self::Jumpstream, Self::Handstream, Self::Stamina, Self::Jackspeed,
			Self::Chordjack, Self::Technical]
	}

	/// Iterate all skillsets
	/// 
	/// Useful for tasks that require operating on all skillsets individually
	/// 
	/// ```rust,no_run
	/// # use etterna_base::Skillset7;
	/// # let skillsets: etterna_base::UserSkillsets = unimplemented!();
	/// for ss in Skillset7::iter() {
	/// 	println!("{}: {}", ss, skillsets.get(ss));
	/// }
	/// ```
	pub fn iter() -> impl Iterator<Item=Self> {
		Self::list().iter().copied()
	}

	/// Self-explanatory.
	/// 
	/// ```rust
	/// # use etterna_base::{Skillset7, Skillset8};
	/// assert_eq!(Skillset7::Stream.into_skillset8(), Skillset8::Stream);
	/// assert_eq!(Skillset7::Chordjack.into_skillset8(), Skillset8::Chordjack);
	/// ```
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
	/// assert_eq!(Skillset8::from_user_input("js"), Some(Skillset8::Jumpstream));
	/// assert_eq!(Skillset8::from_user_input("Jacks"), Some(Skillset8::Jackspeed));
	/// assert_eq!(Skillset8::from_user_input("JACKSPEED"), Some(Skillset8::Jackspeed));
	/// assert_eq!(Skillset8::from_user_input("handstreams"), None);
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
	/// 
	/// Useful for tasks that require operating on all skillsets individually
	/// 
	/// ```rust,no_run
	/// # use etterna_base::Skillset8;
	/// # let skillsets: etterna_base::UserSkillsets = unimplemented!();
	/// for &ss in Skillset8::list() {
	/// 	println!("{}: {}", ss, skillsets.get(ss));
	/// }
	/// ```
	pub fn list() -> &'static [Self] {
		&[Self::Overall, Self::Stream, Self::Jumpstream, Self::Handstream, Self::Stamina,
			Self::Jackspeed, Self::Chordjack, Self::Technical]
	}

	/// Iterate all skillsets
	/// 
	/// Useful for tasks that require operating on all skillsets individually
	/// 
	/// ```rust,no_run
	/// # use etterna_base::Skillset8;
	/// # let skillsets: etterna_base::UserSkillsets = unimplemented!();
	/// for ss in Skillset8::iter() {
	/// 	println!("{}: {}", ss, skillsets.get(ss));
	/// }
	/// ```
	pub fn iter() -> impl Iterator<Item=Self> {
		Self::list().iter().copied()
	}

	/// Convert into a Skillset7, converting Overall into None along the way.
	/// 
	/// ```rust
	/// # use etterna_base::{Skillset8, Skillset7};
	/// assert_eq!(Skillset8::Stream.into_skillset7(), Some(Skillset7::Stream));
	/// assert_eq!(Skillset8::Overall.into_skillset7(), None);
	/// ```
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