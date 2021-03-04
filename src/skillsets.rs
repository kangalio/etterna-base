use std::convert::{TryFrom, TryInto};

/// Skillset information, excluding overall
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Skillsets7 {
	pub stream: f32,
	pub jumpstream: f32,
	pub handstream: f32,
	pub stamina: f32,
	pub jackspeed: f32,
	pub chordjack: f32,
	pub technical: f32,
}

impl Skillsets7 {
	pub fn get(&self, ss: Skillset7) -> f32 {
		match ss {
			Skillset7::Stream => self.stream,
			Skillset7::Jumpstream => self.jumpstream,
			Skillset7::Handstream => self.handstream,
			Skillset7::Stamina => self.stamina,
			Skillset7::Jackspeed => self.jackspeed,
			Skillset7::Chordjack => self.chordjack,
			Skillset7::Technical => self.technical,
		}
	}

	pub fn with_overall(&self, overall: f32) -> Skillsets8 {
		Skillsets8 {
			overall,
			stream: self.stream,
			jumpstream: self.jumpstream,
			handstream: self.handstream,
			stamina: self.stamina,
			jackspeed: self.jackspeed,
			chordjack: self.chordjack,
			technical: self.technical,
		}
	}

	pub fn calc_player_overall(&self) -> Skillsets8 {
		let overall = crate::rating_calc::calculate_player_overall(&[
			self.stream,
			self.jumpstream,
			self.handstream,
			self.stamina,
			self.jackspeed,
			self.chordjack,
			self.technical,
		]);

		self.with_overall(overall)
	}

	pub fn calc_player_overall_pre_070(&self) -> Skillsets8 {
		let overall =
			(self.stream
				+ self.jumpstream
				+ self.handstream
				+ self.stamina + self.jackspeed
				+ self.chordjack + self.technical)
				/ 7.0;

		self.with_overall(overall)
	}

	pub fn calc_ssr_overall(&self) -> Skillsets8 {
		let aggregated_skillsets = crate::rating_calc::calculate_score_overall(&[
			self.stream,
			self.jumpstream,
			self.handstream,
			self.stamina,
			self.jackspeed,
			self.chordjack,
			self.technical,
		]);

		let max_skillset = self
			.stream
			.max(self.jumpstream)
			.max(self.handstream)
			.max(self.stamina)
			.max(self.jackspeed)
			.max(self.chordjack)
			.max(self.technical);

		let overall = aggregated_skillsets.max(max_skillset);
		self.with_overall(overall)
	}

	pub fn calc_ssr_overall_pre_070(&self) -> Skillsets8 {
		let max_skillset = self
			.stream
			.max(self.jumpstream)
			.max(self.handstream)
			.max(self.stamina)
			.max(self.jackspeed)
			.max(self.chordjack)
			.max(self.technical);

		self.with_overall(max_skillset)
	}

	pub fn generate<F: FnMut(crate::Skillset7) -> f32>(mut generator: F) -> Self {
		Self {
			stream: (generator)(crate::Skillset7::Stream),
			jumpstream: (generator)(crate::Skillset7::Jumpstream),
			handstream: (generator)(crate::Skillset7::Handstream),
			stamina: (generator)(crate::Skillset7::Stamina),
			jackspeed: (generator)(crate::Skillset7::Jackspeed),
			chordjack: (generator)(crate::Skillset7::Chordjack),
			technical: (generator)(crate::Skillset7::Technical),
		}
	}
}

impl From<Skillsets8> for Skillsets7 {
	fn from(s: Skillsets8) -> Self {
		s.to_skillsets7()
	}
}

/// Skillset information, excluding overall
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Skillsets8 {
	pub overall: f32,
	pub stream: f32,
	pub jumpstream: f32,
	pub handstream: f32,
	pub stamina: f32,
	pub jackspeed: f32,
	pub chordjack: f32,
	pub technical: f32,
}

impl Skillsets8 {
	pub fn get(&self, ss: Skillset8) -> f32 {
		match ss {
			Skillset8::Overall => self.overall,
			Skillset8::Stream => self.stream,
			Skillset8::Jumpstream => self.jumpstream,
			Skillset8::Handstream => self.handstream,
			Skillset8::Stamina => self.stamina,
			Skillset8::Jackspeed => self.jackspeed,
			Skillset8::Chordjack => self.chordjack,
			Skillset8::Technical => self.technical,
		}
	}

	pub fn to_skillsets7(&self) -> Skillsets7 {
		Skillsets7 {
			stream: self.stream,
			jumpstream: self.jumpstream,
			handstream: self.handstream,
			stamina: self.stamina,
			jackspeed: self.jackspeed,
			chordjack: self.chordjack,
			technical: self.technical,
		}
	}

	pub fn generate<F: FnMut(crate::Skillset8) -> f32>(mut generator: F) -> Self {
		Self {
			overall: (generator)(crate::Skillset8::Overall),
			stream: (generator)(crate::Skillset8::Stream),
			jumpstream: (generator)(crate::Skillset8::Jumpstream),
			handstream: (generator)(crate::Skillset8::Handstream),
			stamina: (generator)(crate::Skillset8::Stamina),
			jackspeed: (generator)(crate::Skillset8::Jackspeed),
			chordjack: (generator)(crate::Skillset8::Chordjack),
			technical: (generator)(crate::Skillset8::Technical),
		}
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
		&[
			Self::Stream,
			Self::Jumpstream,
			Self::Handstream,
			Self::Stamina,
			Self::Jackspeed,
			Self::Chordjack,
			Self::Technical,
		]
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
	pub fn iter() -> impl Iterator<Item = Self> {
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
		&[
			Self::Overall,
			Self::Stream,
			Self::Jumpstream,
			Self::Handstream,
			Self::Stamina,
			Self::Jackspeed,
			Self::Chordjack,
			Self::Technical,
		]
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
	pub fn iter() -> impl Iterator<Item = Self> {
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
