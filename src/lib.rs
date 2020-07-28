pub mod structs;
pub use structs::*;

mod wife;
pub use wife::*;

mod rescore;
pub use rescore::*;

mod util;

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