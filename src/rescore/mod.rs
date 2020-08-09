use itertools::izip;

mod matching_scorer;
pub use matching_scorer::MatchingScorer;

mod naive_scorer;
pub use naive_scorer::NaiveScorer;

/// Result of evaluating a [`ScoringSystem`] on a list of notes and hits
/// ([`ScoringSystem::evaluate`])
pub struct ScoringResult {
	wifescore_sum: f32,
	num_judged_notes: u64,
}

/// Trait for a scorer that operates on a single column and evaluates all hits on that column. It
/// needs the entire list of hits available to it at the same time
pub trait ScoringSystem: Sized {
	/// Evaluate the scoring system on the given list of notes and hits. The lists must be sorted
	/// by the hits!
	fn evaluate<W: crate::Wife>(
		note_seconds: &[f32],
		hit_seconds: &[f32],
		judge: &crate::Judge,
	) -> ScoringResult;
}

/// Calculates a wifescore from a list of notes per column and hits per column, plus the mine hits
/// and hold drops. The wifescore algorithm and scoring algorithm used can be chosen via the generic
/// parameters.
pub fn rescore<S, W>(
	note_seconds_columns: &[Vec<f32>; 4],
	hit_seconds_columns: &[Vec<f32>; 4],
	num_mine_hits: u32,
	num_hold_drops: u32,
	judge: &crate::Judge,
) -> crate::Wifescore
where
	S: ScoringSystem,
	W: crate::Wife
{
	let mut wifescore_sum = 0.0;
	let mut num_judged_notes = 0;
	for (note_seconds, hit_seconds) in izip!(note_seconds_columns, hit_seconds_columns) {
		assert!(crate::util::is_sorted(hit_seconds));
		assert!(crate::util::is_sorted(note_seconds));

		let column_scoring_result = S::evaluate::<W>(&note_seconds, &hit_seconds, judge);

		wifescore_sum += column_scoring_result.wifescore_sum;
		num_judged_notes += column_scoring_result.num_judged_notes;
	}

	wifescore_sum += W::MINE_HIT_WEIGHT * num_mine_hits as f32;
	wifescore_sum += W::HOLD_DROP_WEIGHT * num_hold_drops as f32;

	let wifescore = wifescore_sum / num_judged_notes as f32;
	crate::Wifescore::from_proportion(wifescore)
		.expect("Invalid wifescore was generated. Maybe the given notes and hits vectors were empty")
}