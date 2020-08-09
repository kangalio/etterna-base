/// Specification of a judgement level
/// 
/// For detailed information, see
/// [this spreadsheet](https://docs.google.com/spreadsheets/d/1syi5aN6sTiDA2Bs_lzZjsLQ1yCEhxl5EnAd6EsD6cF4)
/// from Foxfire and poco0317
pub struct Judge {
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

	/// Returns whether the deviation is considered a miss or not
	pub fn is_miss(&self, deviation: f32) -> bool {
		return deviation.abs() > self.bad_window;
	}
}

/// Removed from Etterna in 0.69.0
pub static J1: Judge = Judge {
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
pub static J2: Judge = Judge {
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
pub static J3: Judge = Judge {
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
pub static J4: Judge = Judge {
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

pub static J5: Judge = Judge {
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

pub static J6: Judge = Judge {
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

/// Half the timing window of J4
pub static J7: Judge = Judge {
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
pub static J8: Judge = Judge {
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

pub static J9: Judge = Judge {
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