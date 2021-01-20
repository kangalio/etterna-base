#![allow(
	clippy::len_zero,
	clippy::tabs_in_doc_comments,
	clippy::collapsible_if,
	clippy::needless_bool
)]

mod wife;
pub use wife::*;

mod rescore;
pub use rescore::*;

mod rating_calc;
pub use rating_calc::*;

mod note_subsets;
pub use note_subsets::*;

mod structs;
pub use structs::*;

mod skillsets;
pub use skillsets::*;

mod rate;
pub use rate::*;

mod timing_info;
pub use timing_info::*;

mod judge;
pub use judge::*;

pub mod prelude {
	pub use crate::structs::*;
	pub use crate::{Rate, Skillset7, Skillset8, Skillsets7, Skillsets8, Wifescore};
}

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
		pub fn get(&self, skillset: impl Into<crate::Skillset8>) -> $return_type {
			let $self_ = self;
			match skillset.into() {
				crate::Skillset8::Overall => $overall_getter,
				crate::Skillset8::Stream => self.stream,
				crate::Skillset8::Jumpstream => self.jumpstream,
				crate::Skillset8::Handstream => self.handstream,
				crate::Skillset8::Stamina => self.stamina,
				crate::Skillset8::Jackspeed => self.jackspeed,
				crate::Skillset8::Chordjack => self.chordjack,
				crate::Skillset8::Technical => self.technical,
			}
		}
	};
	($return_type:ty, $self_:ident, $overall_getter:expr, $overall_getter_pre_070:expr) => {
		crate::impl_get_skillset!($return_type, $self_, $overall_getter);

		/// Get a specific skillset value. If Overall was requested, use the pre-0.70 algorithm
		/// for calculation.
		pub fn get_pre_070(&self, skillset: impl Into<crate::Skillset8>) -> $return_type {
			let $self_ = self;
			match skillset.into() {
				crate::Skillset8::Overall => $overall_getter_pre_070,
				crate::Skillset8::Stream => self.stream,
				crate::Skillset8::Jumpstream => self.jumpstream,
				crate::Skillset8::Handstream => self.handstream,
				crate::Skillset8::Stamina => self.stamina,
				crate::Skillset8::Jackspeed => self.jackspeed,
				crate::Skillset8::Chordjack => self.chordjack,
				crate::Skillset8::Technical => self.technical,
			}
		}
	};
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
	pub changes: Vec<(T, Skillsets8)>,
}

impl<T: PartialEq + Send> SkillTimeline<T> {
	/// Generate a timeline of player ratings over time. The input is given in form of an iterator
	/// over tuples of each score's group identifier and the score's skillsets.
	///
	/// "What's a group identifier" you might ask. Well, this function doesn't re-calculate the
	/// player's rating for each and every score. That would be wasteful. Instead, scores are
	/// grouped, usually by day, and the rating is re-calculated for each group.
	///
	/// You can use almost any type you want as a group identifier as long as it can be compared
	/// (has a PartialEq impl).
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
	pub fn calculate<I>(iterator: I, pre_070: bool) -> SkillTimeline<T>
	where
		I: IntoIterator<Item = (T, Skillsets7)>,
	{
		let skillset_calc_function = if pre_070 {
			rating_calc::calculate_player_skillset_rating_pre_070
		} else {
			rating_calc::calculate_player_skillset_rating
		};
		let overall_calc_function = if pre_070 {
			Skillsets7::calc_player_overall_pre_070
		} else {
			Skillsets7::calc_player_overall
		};

		let iterator = iterator.into_iter();
		let approx_num_scores = iterator.size_hint().1.unwrap_or(iterator.size_hint().0);
		let mut rating_vectors: [Vec<f32>; 7] = [
			Vec::with_capacity(approx_num_scores),
			Vec::with_capacity(approx_num_scores),
			Vec::with_capacity(approx_num_scores),
			Vec::with_capacity(approx_num_scores),
			Vec::with_capacity(approx_num_scores),
			Vec::with_capacity(approx_num_scores),
			Vec::with_capacity(approx_num_scores),
		];

		// naming of "day" in here is legacy; pretend it says "group" instead (as per docs above)
		let mut day_indices: Vec<(T, usize)> = vec![];
		let mut prev_day_id = None;
		for (day_id, ssr) in iterator {
			rating_vectors[0].push(ssr.stream);
			rating_vectors[1].push(ssr.jumpstream);
			rating_vectors[2].push(ssr.handstream);
			rating_vectors[3].push(ssr.stamina);
			rating_vectors[4].push(ssr.jackspeed);
			rating_vectors[5].push(ssr.chordjack);
			rating_vectors[6].push(ssr.technical);

			if let Some(prev_day_id) = prev_day_id.take() {
				if prev_day_id != day_id {
					day_indices.push((prev_day_id, rating_vectors[0].len()));
				}
			}
			prev_day_id = Some(day_id);
		}
		if let Some(prev_day_id) = prev_day_id {
			day_indices.push((prev_day_id, rating_vectors[0].len()));
		}

		let changes = par_iter_maybe(day_indices)
			.map(|(day_id, i)| {
				(
					day_id,
					overall_calc_function(&Skillsets7 {
						stream: (skillset_calc_function)(&rating_vectors[0][..i]),
						jumpstream: (skillset_calc_function)(&rating_vectors[1][..i]),
						handstream: (skillset_calc_function)(&rating_vectors[2][..i]),
						stamina: (skillset_calc_function)(&rating_vectors[3][..i]),
						jackspeed: (skillset_calc_function)(&rating_vectors[4][..i]),
						chordjack: (skillset_calc_function)(&rating_vectors[5][..i]),
						technical: (skillset_calc_function)(&rating_vectors[6][..i]),
					}),
				)
			})
			.collect();

		Self { changes }
	}
}

#[deprecated(note = "Use SkillTimeline::calculate instead")]
pub fn skill_timeline<I, T>(iterator: I, pre_070: bool) -> SkillTimeline<T>
where
	I: IntoIterator<Item = (T, Skillsets7)>,
	T: PartialEq + Send,
{
	SkillTimeline::calculate(iterator, pre_070)
}
