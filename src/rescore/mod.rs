use itertools::izip;

mod matching_scorer;
pub use matching_scorer::MatchingScorer;

mod naive_scorer;
pub use naive_scorer::NaiveScorer;


pub struct ScoringResult {
	wifescore_sum: f32,
	num_judged_notes: u64,
}

/// Trait for a scorer that operates on a single column and evaluates all hits on that column. It
/// needs the entire list of hits available to it at the same time
pub trait ScoringSystem: Sized {
	/// Evaluate the scoring system on the given list of notes and hits. The lists must be sorted!
	fn evaluate<W: crate::Wife>(note_seconds: &[f32], hit_seconds: &[f32]) -> ScoringResult;
}

pub trait SimpleReplay {
	fn iter_deviations(&self) -> Box<dyn '_ + Iterator<Item = f32>>;

	// TODO
	// fn rescore<W: crate::Wife>(&self) -> crate::Wifescore { todo!() }

	/// Finds the longest combo of notes in this replay such that all notes in the combo yield true
	/// when their deviation is supplied into the given closure.
	/// 
	/// The note deviations passed into the closure will always be positive.
	/// 
	/// # Example
	/// Find the longest marvelous combo:
	/// ```rust
	/// let longest_marvelous_combo = replay.longest_combo(|d| d < 0.0225);
	/// ```
	fn longest_combo(&self, mut note_filter: impl FnMut(f32) -> bool) -> u32 {
		crate::util::longest_true_sequence(
			self
				.iter_deviations()
				.map(|d| note_filter(d.abs()))
		)
	}
}

/// Calculates a wifescore from a list of notes per column and hits per column, plus the mine hits
/// and hold drops. The wifescore algorithm and scoring algorithm used can be chosen via the generic
/// parameters.
pub fn rescore<S, W>(
	note_seconds_columns: &[Vec<f32>; 4],
	hit_seconds_columns: &[Vec<f32>; 4],
	num_mine_hits: u32,
	num_hold_drops: u32,
) -> crate::Wifescore
where
	S: ScoringSystem,
	W: crate::Wife
{
	let mut wifescore_sum = 0.0;
	let mut num_judged_notes = 0;
	for (note_seconds, hit_seconds) in izip!(note_seconds_columns, hit_seconds_columns) {
		let column_scoring_result = S::evaluate::<W>(&note_seconds, &hit_seconds);

		wifescore_sum += column_scoring_result.wifescore_sum;
		num_judged_notes += column_scoring_result.num_judged_notes;
	}

	wifescore_sum += W::MINE_HIT_WEIGHT * num_mine_hits as f32;
	wifescore_sum += W::HOLD_DROP_WEIGHT * num_hold_drops as f32;

	let wifescore = wifescore_sum / num_judged_notes as f32;
	crate::Wifescore::from_proportion(wifescore)
		.expect("Invalid wifescore was generated. Maybe the given notes and hits vectors were empty")
}