#![allow(dead_code)]

pub mod prop;
pub mod nat;
mod examples;

pub use prop::Prop;
pub use prop::True;
pub use prop::False;

//==================================================================================================
// Infeasible code

/// Can only be called by proving the False.
pub fn by_contradiction(_: False) {}

#[macro_export]
macro_rules! infeasible {
    ($proof:expr) => {
        {
            $crate::by_contradiction($proof); // check the proof of False
            unimplemented!()
        }
    };
}
