mod wife2;
pub use wife2::*;
mod wife3;
pub use wife3::*;

/// Trait that abstracts over wifescore algorithm - the algorithm that turns a hit deviation into a
/// score percent
pub trait Wife {
	/// Score value which is associated with mine hits (typically negative)
	const MINE_HIT_WEIGHT: f32;
	/// Score value which is associated with hold drops (typically negative)
	const HOLD_DROP_WEIGHT: f32;
	/// Score value which is associated with missed notes (typically negative)
	const MISS_WEIGHT: f32;

	/// Calculate a wifescore by the note deviation, which can be positive or negative
	fn calc_deviation(deviation: f32, judge: &crate::Judge) -> f32;

	/// Calculate the wifescore for a note hit
	fn calc(hit: crate::Hit, judge: &crate::Judge) -> f32 {
		match hit {
			crate::Hit::Hit { deviation } => Self::calc_deviation(deviation, judge),
			crate::Hit::Miss => Self::MISS_WEIGHT,
		}
	}

	/// Utility function to apply this wifescore algorithm to a list of note hits, mine hits and
	/// hold drops.
	/// 
	/// Returns None if the `note_hits` iterator is empty
	fn apply(
		note_hits: impl IntoIterator<Item=crate::Hit>,
		num_mine_hits: u32,
		num_hold_drops: u32,
		judge: &crate::Judge
	) -> Option<crate::Wifescore> {
		let mut num_note_hits = 0;
		let mut wifescore_sum = 0.0;
		for hit in note_hits {
			wifescore_sum += Self::calc(hit, judge);
			num_note_hits += 1;
		}

		wifescore_sum += num_mine_hits as f32 * Self::MINE_HIT_WEIGHT;
		wifescore_sum += num_hold_drops as f32 * Self::HOLD_DROP_WEIGHT;

		crate::Wifescore::from_proportion(wifescore_sum / num_note_hits as f32)
	}
}

/// Utility function to calculate a Wife2 score for a single hit deviation
pub fn wife2(hit: impl Into<crate::Hit>, judge: &crate::Judge) -> f32 {
	Wife2::calc(hit.into(), judge)
}

/// Utility function to calculate a Wife3 score for a single hit deviation
pub fn wife3(hit: impl Into<crate::Hit>, judge: &crate::Judge) -> f32 {
	Wife3::calc(hit.into(), judge)
}

#[cfg(test)]
mod tests {
	use super::*;
	use itertools::izip;

	#[test]
	fn test_wife() {
		const TEST_DEVIATIONS: [f32; 8] = [0.0, 0.03, 0.15, 0.179, 0.18, 0.2, 0.26, 10.0];
		const TEST_JUDGES: [&crate::Judge; 3] = [crate::J1, crate::J4, crate::J9];
		const TEST_WIFE_FNS: [fn(f32, &crate::Judge) -> f32; 2] = [wife2, wife3];

		let test_data: &[[[f32; TEST_WIFE_FNS.len()]; TEST_JUDGES.len()]; TEST_DEVIATIONS.len()] = &[
			[[ 1.00000000,  1.00000000], [ 1.00000000,  1.00000000], [ 1.00000000,  1.00000000]],
			[[ 0.99542332,  0.99242789], [ 0.97769690,  0.97078007], [-2.38148451, -1.75365114]],
			[[-0.43687677, -0.93580455], [-2.38148451, -2.03260875], [-4.00000000, -2.75000000]],
			[[-1.21131277, -1.37423515], [-3.18280602, -2.72608685], [-4.00000000, -2.75000000]],
			[[-1.23852777, -1.38935339], [-3.20406675, -2.75000000], [-4.00000000, -2.75000000]],
			[[-1.77302504, -1.69171941], [-3.54750085, -2.75000000], [-4.00000000, -2.75000000]],
			[[-3.05441713, -2.59881711], [-3.94453931, -2.75000000], [-4.00000000, -2.75000000]],
			[[-4.00000000, -2.75000000], [-4.00000000, -2.75000000], [-4.00000000, -2.75000000]],
		];

		for (&deviation, test_data) in izip!(&TEST_DEVIATIONS, test_data) {
			for (&judge, test_data) in izip!(&TEST_JUDGES, test_data) {
				for (&wife_fn, &expected) in izip!(&TEST_WIFE_FNS, test_data) {
					assert!((wife_fn(deviation, judge) - expected).abs() < 0.00000001);
					assert!((wife_fn(-deviation, judge) - expected).abs() < 0.00000001);
				}
			}
		}
	}
}