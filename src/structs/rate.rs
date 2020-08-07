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
	// TODO: Rework this to not rely on float parsing but parse the digits directly
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

impl From<f32> for Rate {
    fn from(value: f32) -> Self {
		Self::from_f32(value).expect("Invalid rate string")
    }
}

impl std::str::FromStr for Rate {
	type Err = ();
	
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s).ok_or(())
    }
}

impl std::ops::Add for Rate {
	type Output = Self;
	
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_x20(self.x20 + rhs.x20)
    }
}

impl std::ops::Sub for Rate {
	type Output = Self;
	
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_x20(self.x20 - rhs.x20)
    }
}

impl std::ops::AddAssign for Rate {
    fn add_assign(&mut self, other: Self) {
        self.x20 += other.x20;
    }
}

impl std::ops::SubAssign for Rate {
    fn sub_assign(&mut self, other: Self) {
        self.x20 -= other.x20;
    }
}