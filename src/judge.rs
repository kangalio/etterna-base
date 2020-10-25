/// Specification of a judgement level
/// 
/// For detailed information, see
/// [this spreadsheet](https://docs.google.com/spreadsheets/d/1syi5aN6sTiDA2Bs_lzZjsLQ1yCEhxl5EnAd6EsD6cF4)
/// from Foxfire and poco0317
pub struct Judge {
	pub name: &'static str,
	pub marvelous_window: f32,
	pub perfect_window: f32,
	pub great_window: f32,
	pub good_window: f32,
	pub bad_window: f32,
	pub hold_window: f32,
	pub roll_window: f32,
	/// This is the window in which you can hit a mine, assuming no notes are prioritized. This is a
	/// +/- value.
	/// 
	/// Before universal mine timing the mine window was equal to the current judge's great window.
	pub mine_window: f32,
	pub(crate) timing_scale: f32,
}

impl Judge {
	/// Classifies a tap deviation in seconds to a judgement. The parameter can be negative.
	pub fn classify(&self, deviation: f32) -> crate::TapJudgement {
		let deviation = deviation.abs();

		if deviation <= self.marvelous_window {
			crate::TapJudgement::Marvelous
		} else if deviation <= self.perfect_window {
			crate::TapJudgement::Perfect
		} else if deviation <= self.great_window {
			crate::TapJudgement::Great
		} else if deviation <= self.good_window {
			crate::TapJudgement::Good
		} else if deviation <= self.bad_window {
			crate::TapJudgement::Bad
		} else {
			crate::TapJudgement::Miss
		}
	}

	/// Whether the given deviation is a combo breaker (CB)
	pub fn is_cb(&self, deviation: f32) -> bool {
		deviation <= self.great_window
	}

	/// Whether the given deviation is considered marvelous
	pub fn is_marv(&self, deviation: f32) -> bool {
		self.classify(deviation) == crate::TapJudgement::Marvelous
	}

	/// Whether the given deviation is considered perfect
	pub fn is_perf(&self, deviation: f32) -> bool {
		self.classify(deviation) == crate::TapJudgement::Perfect
	}

	/// Whether the given deviation is considered great
	pub fn is_great(&self, deviation: f32) -> bool {
		self.classify(deviation) == crate::TapJudgement::Great
	}

	/// Whether the given deviation is considered good
	pub fn is_good(&self, deviation: f32) -> bool {
		self.classify(deviation) == crate::TapJudgement::Good
	}

	/// Whether the given deviation is considered bad
	pub fn is_bad(&self, deviation: f32) -> bool {
		self.classify(deviation) == crate::TapJudgement::Bad
	}

	/// Whether the given deviation is considered a bad
	pub fn is_miss(&self, deviation: f32) -> bool {
		self.classify(deviation) == crate::TapJudgement::Miss
	}
}

/// Removed from Etterna in 0.69.0
pub const J1: &Judge = &Judge {
	name: "J1",
	marvelous_window: 0.03375,
	perfect_window: 0.0675,
	great_window: 0.135,
	good_window: 0.2025,
	bad_window: 0.27,
	hold_window: 0.375,
	roll_window: 0.75,
	mine_window: 0.075,
	timing_scale: 1.50,
};

/// Removed from Etterna in 0.69.0
pub const J2: &Judge = &Judge {
	name: "J2",
	marvelous_window: 0.029925,
	perfect_window: 0.05985,
	great_window: 0.1197,
	good_window: 0.17955,
	bad_window: 0.2394,
	hold_window: 0.3325,
	roll_window: 0.665,
	mine_window: 0.075,
	timing_scale: 1.33,
};

/// Removed from Etterna in 0.69.0
pub const J3: &Judge = &Judge {
	name: "J3",
	marvelous_window: 0.0261,
	perfect_window: 0.0522,
	great_window: 0.1044,
	good_window: 0.1566,
	bad_window: 0.2088,
	hold_window: 0.29,
	roll_window: 0.58,
	mine_window: 0.075,
	timing_scale: 1.16,
};

/// Default judge for official scoring
pub const J4: &Judge = &Judge {
	name: "J4",
	marvelous_window: 0.0225,
	perfect_window: 0.045,
	great_window: 0.09,
	good_window: 0.135,
	bad_window: 0.18,
	hold_window: 0.25,
	roll_window: 0.5,
	mine_window: 0.075,
	timing_scale: 1.00,
};

/// Used by some as their go-to judge
pub const J5: &Judge = &Judge {
	name: "J5",
	marvelous_window: 0.0189,
	perfect_window: 0.0378,
	great_window: 0.0756,
	good_window: 0.1134,
	/// Before J4 boo window lock: 151.2ms
	bad_window: 0.18,
	hold_window: 0.21,
	roll_window: 0.42,
	timing_scale: 0.84,
	mine_window: 0.075,
};

/// Sometimes used for accuracy training
pub const J6: &Judge = &Judge {
	name: "J6",
	marvelous_window: 0.01485,
	perfect_window: 0.0297,
	great_window: 0.0594,
	good_window: 0.0891,
	/// Before J4 boo window lock: 118.8ms
	bad_window: 0.18,
	hold_window: 0.165,
	roll_window: 0.33,
	timing_scale: 0.66,
	mine_window: 0.075,
};

/// Half the timing window of J4. Common for accuracy training
pub const J7: &Judge = &Judge {
	name: "J7",
	marvelous_window: 0.01125,
	perfect_window: 0.0225,
	great_window: 0.045,
	good_window: 0.0675,
	/// Before J4 boo window lock: 90ms
	bad_window: 0.18,
	hold_window: 0.125,
	roll_window: 0.25,
	timing_scale: 0.50,
	mine_window: 0.075,
};

/// Half the timing window of J6
pub const J8: &Judge = &Judge {
	name: "J8",
	marvelous_window: 0.007425,
	perfect_window: 0.01485,
	great_window: 0.0297,
	good_window: 0.04455,
	/// Before J4 boo window lock: 59.4ms
	bad_window: 0.18,
	hold_window: 0.0825,
	/// Before J7 Roll Lock: 165ms
	timing_scale: 0.33,
	roll_window: 0.25,
	mine_window: 0.075,
};

/// Also called "Justice". Originally added to the game as a joke
pub const J9: &Judge = &Judge {
	name: "J9",
	marvelous_window: 0.0045,
	perfect_window: 0.009,
	great_window: 0.018,
	good_window: 0.027,
	/// Before J4 boo window lock: 36ms
	bad_window: 0.18,
	hold_window: 0.05,
	/// Before J7 Roll lock: 100ms
	timing_scale: 0.20,
	roll_window: 0.25,
	mine_window: 0.075,
};