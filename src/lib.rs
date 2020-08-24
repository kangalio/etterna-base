#![allow(clippy::len_zero, clippy::tabs_in_doc_comments, clippy::collapsible_if, clippy::needless_bool)]

pub mod structs;
pub use structs::*;

mod wife;
pub use wife::*;

mod rescore;
pub use rescore::*;

mod rating_calc;
pub use rating_calc::*;

mod note_subsets;
pub use note_subsets::*;

mod savegame;
pub use savegame::*;

mod pattern;
pub use pattern::*;

mod util;

#[cfg(all(feature = "rayon", not(feature = "parallel")))]
compile_error!("Use the `parallel` feature flag instead of `rayon`");

#[cfg(feature = "parallel")]
use rayon::iter::ParallelIterator;

// this needs to be here for some reason, and it also needs to be publically accessible because MACROS
#[doc(hidden)]
#[macro_export]
macro_rules! impl_get_skillset {
	($return_type:ty, $self_:ident, $overall_getter:expr) => {
		/// Get a specific skillset value
		pub fn get(&self, skillset: impl Into<Skillset8>) -> $return_type {
			let $self_ = self;
			match skillset.into() {
				Skillset8::Overall => $overall_getter,
				Skillset8::Stream => self.stream,
				Skillset8::Jumpstream => self.jumpstream,
				Skillset8::Handstream => self.handstream,
				Skillset8::Stamina => self.stamina,
				Skillset8::Jackspeed => self.jackspeed,
				Skillset8::Chordjack => self.chordjack,
				Skillset8::Technical => self.technical,
			}
		}
	};
	($return_type:ty, $self_:ident, $overall_getter:expr, $overall_getter_pre_070:expr) => {
		crate::impl_get_skillset!($return_type, $self_, $overall_getter);

		/// Get a specific skillset value. If Overall was requested, use the pre-0.70 algorithm
		/// for calculation.
		pub fn get_pre_070(&self, skillset: impl Into<Skillset8>) -> $return_type {
			let $self_ = self;
			match skillset.into() {
				Skillset8::Overall => $overall_getter_pre_070,
				Skillset8::Stream => self.stream,
				Skillset8::Jumpstream => self.jumpstream,
				Skillset8::Handstream => self.handstream,
				Skillset8::Stamina => self.stamina,
				Skillset8::Jackspeed => self.jackspeed,
				Skillset8::Chordjack => self.chordjack,
				Skillset8::Technical => self.technical,
			}
		}
	}
}

// do we even need this?
// #[doc(hidden)]
// #[macro_export]
// macro_rules! impl_get_tap_judgement {
// 	($return_type:ty) => {
// 		pub fn get(&self, judgement: TapJudgement) -> $return_type {

// 		}
// 	}
// }

#[cfg(feature = "parallel")]
fn par_iter_maybe<I>(collection: I) -> I::Iter
where
	I: rayon::iter::IntoParallelIterator,
	I::Item: Send,
{
	collection.into_par_iter()
}
#[cfg(not(feature = "parallel"))]
fn par_iter_maybe<I>(collection: I) -> I::IntoIter
where
	I: IntoIterator,
	I::Item: Send,
{
	collection.into_iter()
}

/// Representation of a player's ratings over time. See [`skill_timeline`]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillTimeline<T> {
	pub changes: Vec<(T, UserSkillsets)>,
}

/// Generate a timeline of player ratings over time. The input is given in form of an iterator
/// over tuples of each score's day identifier and the score's skillsets.
/// 
/// "What's a day identifier" you might ask. Well, this function doesn't re-calculate the player's
/// rating for each and every score. That would be wasteful. Instead, scores are grouped, usually
/// by day, and the rating is re-calculated for each day.
/// 
/// You can use almost any type you want as a day identifier as long as it can be compared (has a
/// PartialEq impl).
/// 
/// You can either use the current, 0.70+ algorithm, or the old algorithm from older game versions.
/// For that, use the `pre_070` parameter.
/// 
/// ```rust,ignore
/// scores = &[
/// 	("2020-08-05", ChartSkillsets { ... }),
/// 	("2020-08-05", ChartSkillsets { ... }),
/// 	("2020-08-05", ChartSkillsets { ... }),
/// 	("2020-08-06", ChartSkillsets { ... }),
/// 	("2020-08-06", ChartSkillsets { ... }),
/// ];
/// 
/// let timeline = skill_timeline(scores, false);
/// assert_eq!(timeline.changes.len(), 2);
/// ```
pub fn skill_timeline<'a, I, T, S>(iterator: I, pre_070: bool) -> SkillTimeline<T>
where
	I: IntoIterator<Item = (T, S)>,
	T: 'a + PartialEq + Copy + Send,
	S: std::borrow::Borrow<ChartSkillsets>,
{
	use itertools::Itertools;

	let skillset_calculation_function = if pre_070 {
		rating_calc::calculate_player_skillset_rating_pre_070
	} else {
		rating_calc::calculate_player_skillset_rating
	};

	let mut rating_vectors: [Vec<f32>; 7] =
		[vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
	
	let mut day_indices: Vec<(T, usize)> = vec![];
	
	let grouped_by_day = iterator.into_iter().group_by(|&(day_id, ref _ssr)| day_id);
	for (day_id, group) in &grouped_by_day {
		for (_day_id, ssr) in group {
			let ssr = ssr.borrow();
			rating_vectors[0].push(ssr.stream);
			rating_vectors[1].push(ssr.jumpstream);
			rating_vectors[2].push(ssr.handstream);
			rating_vectors[3].push(ssr.stamina);
			rating_vectors[4].push(ssr.jackspeed);
			rating_vectors[5].push(ssr.chordjack);
			rating_vectors[6].push(ssr.technical);
		}
		day_indices.push((day_id, rating_vectors[0].len()));
	}

	let changes = par_iter_maybe(day_indices)
		.map(|(day_id, i)| (day_id, UserSkillsets {
			stream: (skillset_calculation_function)(&rating_vectors[0][..i]),
			jumpstream: (skillset_calculation_function)(&rating_vectors[1][..i]),
			handstream: (skillset_calculation_function)(&rating_vectors[2][..i]),
			stamina: (skillset_calculation_function)(&rating_vectors[3][..i]),
			jackspeed: (skillset_calculation_function)(&rating_vectors[4][..i]),
			chordjack: (skillset_calculation_function)(&rating_vectors[5][..i]),
			technical: (skillset_calculation_function)(&rating_vectors[6][..i]),
		}))
		.collect();

	SkillTimeline { changes }
}