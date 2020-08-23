//! https://discord.com/channels/339597420239519755/389194939881488385/735175202006237344
//! The following implementations are bit-accurate to the Etterna game code as of 2020-07-21

fn is_rating_okay(rating: f32, ssrs: &[f32], delta_multiplier: f32) -> bool {
	// Notice the somewhat peculiar usage of f32 and f64 in here. That's to mirror the C++
	// implementation as closely as possible - we thrive for bit-accuracy after all

	let max_power_sum: f64 = 2f64.powf(rating as f64 * 0.1);
	
	let power_sum: f64 = ssrs.iter()
		.map(|&ssr| (2.0 / libm::erfcf(delta_multiplier * (ssr - rating)) - 2.0) as f64)
		.filter(|&x| x > 0.0)
		.sum();
	
	power_sum < max_power_sum
}

/*
The idea is the following: we try out potential skillset rating values
until we've found the lowest rating that still fits (I've called that
property 'okay'-ness in the code).
How do we know whether a potential skillset rating fits? We give each
score a "power level", which is larger when the skillset rating of the
specific score is high. Therefore, the user's best scores get the
highest power levels.
Now, we sum the power levels of each score and check whether that sum
is below a certain limit. If it is still under the limit, the rating
fits (is 'okay'), and we can try a higher rating. If the sum is above
the limit, the rating doesn't fit, and we need to try out a lower
rating.
*/

fn calc_rating(
	ssrs: &[f32],
	final_multiplier: f32,
	delta_multiplier: f32,
) -> f32 {
	let num_iters: u32 = 11; // if needed, make this a parameter in the future

	let mut rating: f32 = 0.0;
	let mut resolution: f32 = 10.24;
	
	// Repeatedly approximate the final rating, with better resolution
	// each time
	for _ in 0..num_iters {
		// Find lowest 'okay' rating with certain resolution
		while !is_rating_okay(rating + resolution, ssrs, delta_multiplier) {
			rating += resolution;
		}

		// Now, repeat with smaller resolution for better approximation
		resolution /= 2.0;
	}
	// Always be ever so slightly above the target value instead of below
	rating += resolution * 2.0;

	rating * final_multiplier
}

/// Calculate a score's overall difficulty from the score's seven individual skillsets.
/// 
/// `AggregateRatings` in Etterna game code:
/// https://github.com/etternagame/etterna/blob/0b7a28d2371798a8138e78e5789d0014b16b4534/src/Etterna/MinaCalc/MinaCalc.cpp#L194-L199, 
/// https://github.com/etternagame/etterna/blob/0b7a28d2371798a8138e78e5789d0014b16b4534/src/Etterna/MinaCalc/MinaCalcHelpers.h#L40-L58
pub fn calculate_score_overall(skillsets: &[f32; 7]) -> f32 {
	calc_rating(skillsets, 1.11, 0.25)
}

/// Calculate a player's skillset rating from the individual scores' skillset ratings
/// 
/// `AggregateSSRs` in Etterna game code:
/// https://github.com/etternagame/etterna/blob/0b7a28d2371798a8138e78e5789d0014b16b4534/src/Etterna/Singletons/ScoreManager.cpp#L808-L837
pub fn calculate_player_skillset_rating(ssrs: &[f32]) -> f32 {
	calc_rating(ssrs, 1.05, 0.1)
}

/// This is the pre-0.70 variant of [`calculate_player_skillset_rating`].
pub fn calculate_player_skillset_rating_pre_070(ssrs: &[f32]) -> f32 {
	calc_rating(ssrs, 1.04, 0.1)
}

/// Calculate a player's overall rating from the player's seven individual skillset ratings.
/// 
/// `AggregateSkillsets` in Etterna game code:
/// https://github.com/etternagame/etterna/blob/0b7a28d2371798a8138e78e5789d0014b16b4534/src/Etterna/Singletons/ScoreManager.cpp#L763-L806
pub fn calculate_player_overall(skillsets: &[f32; 7]) -> f32 {
	calc_rating(skillsets, 1.125, 0.1)
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_everything() {
		// These test values are derived with a C++ program containing standalone versions of the
		// actual algorithm code snippets from Etterna itself
		let test_values: &[([f32; 7], f32, f32, f32, f32)] = &[
			([21.0, 24.0, 23.0, 14.0, 17.0, 25.0, 24.0], 25.27470016, 21.94499779, 21.73599815, 23.51249886),
			([25.0, 23.0, 30.0, 30.0, 17.0, 25.0, 24.0], 30.51390076, 25.70400047, 25.45919991, 27.54000092),
			([26.0, 23.0, 29.0, 15.0, 19.0, 22.0, 25.0], 28.62689972, 24.01350021, 23.78479958, 25.72875023),
			([23.0, 24.0, 24.0, 23.0, 25.0, 24.0, 23.0], 25.46340179, 22.68000031, 22.46399879, 24.30000114),
			([10.0, 100.0, 42.0, 69.0, 3.0, 88.0, 50.0], 101.82029724, 85.09198761, 84.28159332, 91.16999054),
		];

		for &(numbers, s_overall, p_ss, p_ss_old, p_overall) in test_values {
			// yeah, I am doing == with floats. That's because I thrive for bit-perfect accuracy
			// on these functions.
			assert_eq!(calculate_score_overall(&numbers), s_overall);
			assert_eq!(calculate_player_skillset_rating(&numbers), p_ss);
			assert_eq!(calculate_player_skillset_rating_pre_070(&numbers), p_ss_old);
			assert_eq!(calculate_player_overall(&numbers), p_overall);
		}
	}
}