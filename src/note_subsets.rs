/// Information about a combo found by [`find_fastest_note_subset`]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FastestComboInfo {
	pub start_second: f32,
	pub end_second: f32,
	pub length: u32,
	pub speed: f32,
}

/// This function finds the fastest note subset (where the number of notes in the subset is in
/// `min_num_notes..=max_num_notes`). The function operates on a single lane only!
/// 
/// ~~The caller still has to scale the returned NPS by the music rate~~ (only applies to
/// etterna-graph, where the note seconds are always in 1.00x)
/// 
/// `seconds` must be sorted!
pub fn find_fastest_note_subset(
	seconds: &[f32],
	min_num_notes: u32,
	max_num_notes: u32, // inclusive
) -> FastestComboInfo {
	assert!(crate::util::is_sorted(seconds));
	assert!(max_num_notes >= min_num_notes);

	let mut fastest = FastestComboInfo {
		start_second: 0.0, end_second: 0.0, length: 0, // dummy values
		speed: 0.0,
	};
	
	if seconds.len() <= min_num_notes as usize { return fastest }
	
	// Do a moving average for every possible subset length (except the large lengths cuz it's
	// unlikely that there'll be something relevant there)
	let end_n = std::cmp::min(seconds.len(), max_num_notes as usize + 1);
	for n in (min_num_notes as usize)..end_n {
		for i in 0..(seconds.len() - n) {
			let end_i = i + n;
			let nps: f32 = (end_i - i) as f32 / (seconds[end_i] - seconds[i]);
			
			// we do >= because than we can potentially catch later - longer - subsets as well.
			// a 30 NPS subset is more impressive at window size 110 than at window size 100.
			if nps >= fastest.speed {
				fastest = FastestComboInfo {
					length: n as u32,
					start_second: seconds[i],
					end_second: seconds[end_i],
					speed: nps,
				};
			}
		}
	}
	
	fastest
}

/// This function finds the "best" note subset (where the number of notes in the subset is in
/// `min_num_notes..=max_num_notes`). The function operates on a single lane only!
/// 
/// How is "best" defined? It's NPS multiplied by Wife points. For example, a sequence of 50 notes
/// in the timespan of 10 seconds, with a wifescore of 80% yields a value of 4.
/// 
/// ~~The caller still has to scale the returned speed value by the music rate~~ (only applies to
/// etterna-graph, where the note seconds are always in 1.00x)
/// 
/// `seconds` must be sorted, and in the same order as `wife_pts`!
pub fn find_fastest_note_subset_wife_pts(
	seconds: &[f32],
	min_num_notes: u32,
	max_num_notes: u32, // inclusive
	wife_pts: &[f32],
) -> FastestComboInfo {
	assert!(wife_pts.len() == seconds.len());
	assert!(crate::util::is_sorted(seconds));
	assert!(max_num_notes >= min_num_notes);
	
	let mut fastest = FastestComboInfo {
		start_second: 0.0, end_second: 0.0, length: 0, // dummy values
		speed: 0.0,
	};
	
	if seconds.len() <= min_num_notes as usize {
		// If the combo is too short to detect any subsets, we return early
		return fastest;
	}
	
	let mut wife_pts_sum_start = wife_pts[0..min_num_notes as usize].iter().sum();
	
	// Do a moving average for every possible subset length
	let end_n = std::cmp::min(seconds.len(), max_num_notes as usize + 1);
	for n in (min_num_notes as usize)..end_n {
		// Instead of calculating the sum of the local wife_pts window for every iteration, we keep
		// a variable to it and simply update it on every iteration instead -> that's faster
		let mut wife_pts_sum: f32 = wife_pts_sum_start;
		
		for i in 0..(seconds.len() - n) {
			let end_i = i + n;
			let mut nps: f32 = (end_i - i) as f32 / (seconds[end_i] - seconds[i]);
			
			nps *= wife_pts_sum / n as f32; // multiply by wife points
			
			if nps >= fastest.speed { // why >=? see other note subset function
				fastest = FastestComboInfo {
					length: n as u32,
					start_second: seconds[i],
					end_second: seconds[end_i],
					speed: nps,
				};
			}
			
			// Move the wife_pts_sum window one place forward
			wife_pts_sum -= wife_pts[i];
			wife_pts_sum += wife_pts[end_i];
		}
		
		// Update the initial window sum
		wife_pts_sum_start += wife_pts[n];
	}
	
	fastest
}

/// Find the fastest combo within the score. It searched only for combos whose lengths lie inside
/// `min_num_notes..=max_num_notes`.
/// 
/// The `are_cbs` iterator must yield as many elements as `seconds` and `wife_pts` (if present)
/// have.
/// 
/// If `wife_pts` is provided, the nps will be multiplied by wife pts. the 'nps' is practically
/// 'wife points per second' then
pub fn find_fastest_combo_in_score<I, T>(
	seconds: &[f32],
	are_cbs: impl IntoIterator<Item=bool>,
	min_num_notes: u32,
	max_num_notes: u32,
	wife_pts: Option<&[f32]>,
	rate: f32,
) -> FastestComboInfo {
	assert!(crate::util::is_sorted(seconds));
	assert!(max_num_notes >= min_num_notes);
	if let Some(wife_pts) = wife_pts {
		assert_eq!(seconds.len(), wife_pts.len());
	}
	
	// The nps track-keeping here is ignoring rate! rate is only applied at the end
	let mut fastest_combo = FastestComboInfo::default();
	
	let mut combo_start_i: Option<usize> = Some(0);
	
	// is called on every cb (cuz that ends a combo) and at the end (cuz that also ends a combo)
	let mut trigger_combo_end = |combo_end_i| {
		if let Some(combo_start_i) = combo_start_i {
			// the position of all notes, in seconds, within a full combo
			let combo = &seconds[combo_start_i..combo_end_i];
			
			let fastest_note_subset;
			if let Some(wife_pts) = wife_pts {
				let wife_pts_slice = &wife_pts[combo_start_i..combo_end_i];
				fastest_note_subset = find_fastest_note_subset_wife_pts(
					combo, min_num_notes, max_num_notes, wife_pts_slice
				);
			} else {
				fastest_note_subset = find_fastest_note_subset(
					combo, min_num_notes, max_num_notes
				);
			}
			
			if fastest_note_subset.speed > fastest_combo.speed {
				fastest_combo = fastest_note_subset;
			}
		}
		combo_start_i = None; // Combo is handled now, a new combo yet has to begin
	};
	
	for (i, is_cb) in are_cbs.into_iter().enumerate() {
		if is_cb {
			trigger_combo_end(i);
		}
	}
	trigger_combo_end(seconds.len());
	
	fastest_combo.speed *= rate;
	
	fastest_combo
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::assert_float_eq;
	
	#[test]
	fn test_find_fastest_note_subset() {
		// This function tests both find_fastest_note_subset and it's wife_pts variant (in which
		// case the wife_pts parameter is a dummy vector filled with 1, so that the wife_pts
		// function should yield identical results to the standard variant). It asserts equality,
		// and also checks if the result length and speed match the expected result
		fn test_the_functions(seconds: &[f32], min_num_notes: u32, max_num_notes: u32,
				expected_length: u32, expected_speed: f32) {
			
			let fastest_subset = find_fastest_note_subset(&seconds,
					min_num_notes, max_num_notes);
			let fastest_wife_pts_subset = find_fastest_note_subset_wife_pts(&seconds,
					min_num_notes, max_num_notes,
					&vec![1.0; seconds.len()]);
			
			assert_float_eq!(fastest_subset.start_second, fastest_wife_pts_subset.start_second;
					epsilon=0.00001);
			assert_float_eq!(fastest_subset.end_second, fastest_wife_pts_subset.end_second;
					epsilon=0.00001);
			assert_eq!(fastest_subset.length, fastest_wife_pts_subset.length);
			assert_float_eq!(fastest_subset.speed, fastest_wife_pts_subset.speed;
					epsilon=0.00001);
			
			assert_eq!(fastest_subset.length, expected_length);
			assert_float_eq!(fastest_subset.speed, expected_speed;
					epsilon=0.00001);
		}
		
		let seconds: &[f32] = &[0.0, 3.0, 5.0, 6.0, 8.0];
		test_the_functions(seconds, 2, 99,
				2, 0.6666666); // should detect [3, 5, 6)
		test_the_functions(seconds, 3, 99,
				3, 0.6); // should detect [3, 5, 6, 8)
		
		// DeltaEpsilon: "Can you find an example where, say, a window of combo 5 will be lower
		// than a window of combo 6." sure, here you go :)
		let seconds: &[f32] = &[0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0];
		test_the_functions(seconds, 5, 6,
				6, 1.5); // note that window size 6 is fastest! not 5
		// when we're restricted to window size 5 at max, the function will obviously not yield
		// the subset with 6 notes. Instead it will be the size-5 window which is, in fact, _slower_
		// than the size-6 window!
		test_the_functions(seconds, 5, 5,
				5, 1.25);
	}
}