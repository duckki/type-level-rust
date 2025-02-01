/// Logical propositions at type level.

/// Any proposition
pub trait Prop {}

//==================================================================================================
// Basic propositions: True & False

/// - True: Logically same as `()` (unit), but it's nice to have a named type.
pub struct True;

mod private_false {
    /// - False: False has no public constructor.
    pub struct False(std::marker::PhantomData<()>);
}

pub use private_false::False;

impl Prop for True {}
impl Prop for False {}

impl Default for True {
    fn default() -> Self {
        True
    }
}

mod prop_tests {
    use super::*;

    fn any_proof<AnyProof: Prop>(_: AnyProof) {}

    fn prop_test() {
        let _: True = True;
        let _: True = True::default();
        any_proof(True::default());
    }

    // Wish: I'd like to uncomment these lines and enforce the expected errors.
    fn error_false_constructor() {
        // any_proof(False(Default::default())); // error
    }
}
