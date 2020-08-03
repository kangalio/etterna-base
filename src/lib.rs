pub mod structs;
pub use structs::*;

mod wife;
pub use wife::*;

mod rescore;
pub use rescore::*;

mod rating_calc;
pub use rating_calc::*;

mod util;

#[cfg(all(feature = "rayon", not(feature = "parallel")))]
compile_error!("Use the `parallel` feature flag instead of `rayon`");

#[cfg(feature = "parallel")]
use rayon::iter::ParallelIterator;

// this needs to be here for some reason, and it also needs to be publically accessible because MACROS
#[doc(hidden)]
#[macro_export]
macro_rules! impl_get8 {
	($struct_type:ty, $return_type:ty, $self_:ident, $overall_getter:expr) => {
		impl $struct_type {
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
		}
	}
}

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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillGraph<T> {
	pub changes: Vec<(T, UserSkillsets)>,
}

pub fn skill_graph<'a, I, T, S>(iterator: I, pre_070: bool) -> SkillGraph<T>
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

	SkillGraph { changes }
}