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
		lane: &crate::NoteAndHitSeconds,
		judge: &crate::Judge,
	) -> ScoringResult;
}

/// Calculates a wifescore from a list of notes per column and hits per column, plus the mine hits
/// and hold drops. The wifescore algorithm and scoring algorithm used can be chosen via the generic
/// parameters.
///
/// Prefer [`rescore_from_note_hits`] if all you need is a judge conversion.
pub fn rescore<S, W>(
	lanes: &[crate::NoteAndHitSeconds; 4],
	num_mine_hits: u32,
	num_hold_drops: u32,
	judge: &crate::Judge,
) -> crate::Wifescore
where
	S: ScoringSystem,
	W: crate::Wife,
{
	let mut wifescore_sum = 0.0;
	let mut num_judged_notes = 0;
	for lane in lanes {
		assert!(crate::util::is_sorted(&lane.hit_seconds));
		assert!(crate::util::is_sorted(&lane.note_seconds));

		let column_scoring_result = S::evaluate::<W>(lane, judge);

		wifescore_sum += column_scoring_result.wifescore_sum;
		num_judged_notes += column_scoring_result.num_judged_notes;
	}

	wifescore_sum += W::MINE_HIT_WEIGHT * num_mine_hits as f32;
	wifescore_sum += W::HOLD_DROP_WEIGHT * num_hold_drops as f32;

	let wifescore = wifescore_sum / num_judged_notes as f32;
	crate::Wifescore::from_proportion(wifescore).expect(
		"Invalid wifescore was generated. Maybe the given notes and hits vectors were empty",
	)
}

/// Calculate a wifescore from a replay's note hits, mine hits and hold drops.
///
/// This function is less generic
/// than `rescore` because you can't choose the scoring system - it's already engrained within the
/// note hits. However, note hits are more easily and reliably obtainable than note seconds and hit
/// seconds columns. Therefore this function should be preferred if only a simple rescore with a
/// different judge is required.
///
/// Returns None if the `note_hits` iterator is empty
///
/// ```rust,no_run
/// let replay: etterna::ReplayV2Fast = todo!();
///
/// let wifescore_on_j7 = etterna::rescore_from_note_hits::<etterna::Wife3, _>(
/// 	replay.notes.iter().map(|note| note.hit),
/// 	replay.num_mine_hits,
/// 	replay.num_hold_drops,
/// 	etterna::J7,
/// );
/// ```
pub fn rescore_from_note_hits<W, I: IntoIterator<Item = crate::Hit>>(
	note_hits: I,
	num_mine_hits: u32,
	num_hold_drops: u32,
	judge: &crate::Judge,
) -> Option<crate::Wifescore>
where
	W: crate::Wife,
{
	W::apply(note_hits, num_mine_hits, num_hold_drops, judge)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_scoring_systems() {
		use crate::Wife as _;

		type Wife = crate::Wife3;
		let judge = crate::J4;
		let wife = |deviation: f32| Wife::calc(crate::Hit::Hit { deviation }, judge);

		let test =
			|note_and_hit_seconds, target_naive_wifescore: f32, target_matching_wifescore: f32| {
				let naive_wifescore = {
					let result = NaiveScorer::evaluate::<Wife>(&note_and_hit_seconds, judge);
					result.wifescore_sum / result.num_judged_notes as f32
				};
				let matching_wifescore = {
					let result = MatchingScorer::evaluate::<Wife>(&note_and_hit_seconds, judge);
					result.wifescore_sum / result.num_judged_notes as f32
				};

				println!("{} == {} ?", naive_wifescore, target_naive_wifescore);
				println!("{} == {} ?", matching_wifescore, target_matching_wifescore);
				assert!((naive_wifescore - target_naive_wifescore).abs() < 0.00001);
				assert!((matching_wifescore - target_matching_wifescore).abs() < 0.00001);
			};

		test(
			crate::NoteAndHitSeconds {
				note_seconds: vec![1.0, 2.0, 3.0, 4.0],
				hit_seconds: vec![1.0, 2.0, 3.0, 4.0],
			},
			wife(0.0),
			wife(0.0),
		);

		test(
			crate::NoteAndHitSeconds {
				note_seconds: vec![1.0, 2.0, 3.0, 4.0],
				hit_seconds: vec![0.9, 3.1, 4.1],
			},
			(wife(0.1) + wife(1.0) + wife(0.1) + wife(0.1)) / 4.0,
			(wife(0.1) + wife(1.0) + wife(0.1) + wife(0.1)) / 4.0,
		);

		test(
			crate::NoteAndHitSeconds {
				note_seconds: vec![0.10, 0.20, 0.30, 0.40],
				hit_seconds: vec![0.09, 0.10, 0.30, 0.40],
			},
			(wife(0.01) + wife(0.10) + wife(0.0) + wife(0.0)) / 4.0,
			(wife(0.0) + wife(0.11) + wife(0.0) + wife(0.0)) / 4.0,
		);

		test(
			crate::NoteAndHitSeconds {
				note_seconds: vec![0.05, 0.10, 0.15, 0.20],
				hit_seconds: vec![0.01, 0.02, 0.03, 0.04, 0.05, 0.10, 0.15, 0.20],
			},
			(wife(0.04) + wife(0.08) + wife(0.12) + wife(0.16)) / 4.0,
			(wife(0.0) * 4.0 /* latter four hits */ + wife(1.0) * 4.0/* first four stray hit punishment */)
				/ 8.0,
		);

		test(
			crate::NoteAndHitSeconds {
				note_seconds: vec![0.05, 0.10, 0.15, 0.20],
				hit_seconds: vec![0.01, 0.02, 0.03, 0.04],
			},
			(wife(0.04) + wife(0.08) + wife(0.12) + wife(0.16)) / 4.0,
			(wife(0.01) + wife(0.07) + wife(0.13) + wife(1.0) /* miss */ + wife(1.0)/* stray tap */)
				/ 5.0,
		);

		test(
			crate::NoteAndHitSeconds {
				note_seconds: vec![0.05, 0.10, 0.15, 0.20],
				hit_seconds: vec![],
			},
			wife(1.0),
			wife(1.0),
		);
	}
}
