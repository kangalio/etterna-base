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
	fn calc(deviation: f32, judge: &crate::Judge) -> f32;

	/// Shorthand function to apply this wifescore algorithm to a list of deviations, mine hits and
	/// hold drops.
	/// 
	/// Misses must be present in the `deviations` slice in form of a `1.000000` value
	fn apply(deviations: &[f32], num_mine_hits: u64, num_hold_drops: u64, judge: &crate::Judge) -> f32 {
		let mut wifescore_sum = 0.0;
		for &deviation in deviations {
			if judge.is_miss(deviation) {
				wifescore_sum += Self::MISS_WEIGHT;
			} else {
				wifescore_sum += Self::calc(deviation, judge);
			}
		}

		wifescore_sum += num_mine_hits as f32 * Self::MINE_HIT_WEIGHT;
		wifescore_sum += num_hold_drops as f32 * Self::HOLD_DROP_WEIGHT;

		wifescore_sum / deviations.len() as f32
	}
}

/// Utility function to calculate a Wife2 score for a single hit deviation
pub fn wife2(deviation: f32, judge: &crate::Judge) -> f32 { Wife2::calc(deviation, judge) }

/// Utility function to calculate a Wife3 score for a single hit deviation
pub fn wife3(deviation: f32, judge: &crate::Judge) -> f32 { Wife3::calc(deviation, judge) }

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

		for (&deviation, test_data) in izip!(TEST_DEVIATIONS, test_data) {
			for (&judge, test_data) in izip!(TEST_JUDGES, test_data) {
				for (&wife_fn, &expected) in izip!(TEST_WIFE_FNS, test_data) {
					assert!((wife_fn(deviation, judge) - expected).abs() < 0.00000001);
					assert!((wife_fn(-deviation, judge) - expected).abs() < 0.00000001);
				}
			}
		}
	}
}