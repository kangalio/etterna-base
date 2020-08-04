use super::{ScoringSystem, ScoringResult};

const DEBUG: bool = false;

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
	fn evaluate<W: crate::Wife>(note_seconds: &[f32], hit_seconds: &[f32]) -> ScoringResult {
		let mut notes: Vec<Note> = note_seconds.iter()
				.map(|&second| Note { second, is_claimed: false })
				.collect();
		
		let mut wifescore_sum = 0.0;
		let mut num_judged_notes: u64 = 0;
		for hit_second in hit_seconds {
			let mut best_note: Option<&mut Note> = None;
			let mut best_note_deviation = f32::INFINITY;
			for note in &mut notes {
				let deviation = (note.second - hit_second).abs();
				if deviation > 0.18 { continue }
				
				if note.is_claimed { continue }
				
				if deviation < best_note_deviation {
					best_note = Some(note);
					best_note_deviation = deviation;
				}
			}

			// If no note was found, this is either a stray tap or the player has mashed SO hard
			// that all the available notes are already claimed by his mashing. In any case, we're
			// not treating such cases in the naive implementation, so we `continue`
			let best_note = match best_note { Some(a) => a, None => continue };

			if DEBUG { println!("{:05.2}: {}", hit_second, best_note_deviation); }

			best_note.is_claimed = true;
			wifescore_sum += W::calc(best_note_deviation);
			num_judged_notes += 1;
		}

		let num_misses = notes.iter().filter(|n| !n.is_claimed).count();
		if DEBUG { println!("wife points no misses: {}", wifescore_sum * 2.0); }
		wifescore_sum += W::MISS_WEIGHT * num_misses as f32;
		if DEBUG { println!("wife points sum w/ misses: {}", wifescore_sum * 2.0); }

		ScoringResult { wifescore_sum, num_judged_notes }
	}
}