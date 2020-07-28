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
	fn calc(deviation: f32) -> f32;

	/// Shorthand function to apply this wifescore algorithm to a list of deviations, mine hits and
	/// hold drops.
	/// 
	/// Misses must be present in the `deviations` slice in form of a `1.000000` value
	fn apply(deviations: &[f32], num_mine_hits: u64, num_hold_drops: u64) -> f32 {
		let mut wifescore_sum = 0.0;
		for &deviation in deviations {
			// if (deviation - 1.0).abs() < 0.0001 { // it's a miss
			if deviation.abs() >= 0.18 - f32::EPSILON { // EO's replay format is not compatible with above
				wifescore_sum += Self::MISS_WEIGHT;
			} else {
				wifescore_sum += Self::calc(deviation);
			}
		}

		wifescore_sum += num_mine_hits as f32 * Self::MINE_HIT_WEIGHT;
		wifescore_sum += num_hold_drops as f32 * Self::HOLD_DROP_WEIGHT;

		wifescore_sum / deviations.len() as f32
	}
}

/// Calculate a score from a hit deviation based on Etterna's Wife2 scoring system
pub fn wife2(deviation: f32) -> f32 { Wife2::calc(deviation) }

/// Calculate a score from a hit deviation based on Etterna's Wife3 scoring system
pub fn wife3(deviation: f32) -> f32 { Wife3::calc(deviation) }