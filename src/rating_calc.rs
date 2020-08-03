//! https://discord.com/channels/339597420239519755/389194939881488385/735175202006237344
//! The following implementations are accurate to the Etterna game code as of 2020-07-21


fn erfc(x: f32) -> f32 { libm::erfc(x as f64) as f32 }

fn is_rating_okay(rating: f32, ssrs: &[f32], delta_multiplier: f32) -> bool {
	let max_power_sum = 2f32.powf(rating * 0.1);
	
	let power_sum: f32 = ssrs.iter()
			.map(|&ssr| 2.0 / erfc(delta_multiplier * (ssr - rating)) - 2.0)
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

/// This was the algorithm pre-0.70
/// 
/// https://github.com/etternagame/etterna/blob/3b99e9dd88/src/Etterna/Singletons/ScoreManager.cpp#L528-L561
pub fn calculate_player_skillset_rating_pre_070(ssrs: &[f32]) -> f32 {
	// not sure if these params are correct; I didn't test them because I don't wannt spend the
	// time and effort to find the old C++ implementation to compare
	calc_rating(ssrs, 1.04, 0.1)
}

/// https://github.com/etternagame/etterna/blob/0b7a28d2371798a8138e78e5789d0014b16b4534/src/Etterna/Singletons/ScoreManager.cpp#L763-L806
pub fn calculate_player_overall(skillsets: &[f32; 7]) -> f32 {
	calc_rating(skillsets, 1.125, 0.1)
}

/// https://github.com/etternagame/etterna/blob/0b7a28d2371798a8138e78e5789d0014b16b4534/src/Etterna/MinaCalc/MinaCalc.cpp#L194-L199
/// https://github.com/etternagame/etterna/blob/0b7a28d2371798a8138e78e5789d0014b16b4534/src/Etterna/MinaCalc/MinaCalcHelpers.h#L40-L58
pub fn calculate_score_overall(skillsets: &[f32; 7]) -> f32 {
	calc_rating(skillsets, 1.11, 0.25)
}

/// https://github.com/etternagame/etterna/blob/0b7a28d2371798a8138e78e5789d0014b16b4534/src/Etterna/Singletons/ScoreManager.cpp#L808-L837
pub fn calculate_player_skillset_rating(ssrs: &[f32]) -> f32 {
	calc_rating(ssrs, 1.05, 0.1)
}