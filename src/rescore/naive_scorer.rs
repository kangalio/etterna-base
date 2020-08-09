use super::{ScoringSystem, ScoringResult};

const DEBUG: bool = false;
const DEBUG_JUDGEMENT_BUG: bool = false;

struct Note {
	second: f32,
	is_claimed: bool,
}

/// Replica of the naive straightforward scoring system as it's usually implemented in mania rhythm
/// games. It maps hits to notes linearly.
/// 
/// This scorer implementation is supposed to exactly replicate Etterna's wifescore results
pub struct NaiveScorer;

impl ScoringSystem for NaiveScorer {
	fn evaluate<W: crate::Wife>(
		note_seconds: &[f32],
		hit_seconds: &[f32],
		judge: &crate::Judge,
	) -> ScoringResult {
		assert!(crate::util::is_sorted(hit_seconds));

		let mut notes: Vec<Note> = note_seconds.iter()
				.map(|&second| Note { second, is_claimed: false })
				.collect();
		
		let mut wifescore_sum = 0.0;
		let mut num_judged_notes: u64 = 0;
		for hit_second in hit_seconds {
			let mut best_note: Option<&mut Note> = None;
			let mut best_note_deviation = f32::INFINITY;
			let mut best_note_deviation_no_abs = 0.0;
			for note in &mut notes {
				let deviation = (hit_second - note.second).abs();
				if deviation > judge.bad_window { continue }
				
				if note.is_claimed { continue }
				
				if deviation < best_note_deviation {
					best_note_deviation_no_abs = hit_second - note.second;
					best_note = Some(note);
					best_note_deviation = deviation;
				}
			}

			// If no note was found, this is either a stray tap or the player has mashed SO hard
			// that all the available notes are already claimed by his mashing. In any case, we're
			// not treating such cases in the naive implementation, so we `continue`
			let best_note = match best_note {
				Some(a) => a,
				None => {
					if DEBUG { println!("No non-claimed notes were found for this hit at {}", hit_second); }
					continue;
				},
			};

			if DEBUG { println!("{:05.2}: {}", hit_second, best_note_deviation); }
			if DEBUG_JUDGEMENT_BUG { print!("{:.5}, ", best_note_deviation_no_abs); }

			best_note.is_claimed = true;
			wifescore_sum += W::calc(best_note_deviation, judge);
			num_judged_notes += 1;
		}
		if DEBUG_JUDGEMENT_BUG { println!(); }

		let num_misses = notes.iter().filter(|n| !n.is_claimed).count();
		wifescore_sum += W::MISS_WEIGHT * num_misses as f32;
		num_judged_notes += num_misses as u64; // I forgot this for the longest time. Full brainfart

		ScoringResult { wifescore_sum, num_judged_notes }
	}
}