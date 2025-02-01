/// Type-level natural number
pub trait Nat {}

// Constructors for Nat: Zero and Succ
pub struct Zero;
pub struct Succ<N>(std::marker::PhantomData<N>);

impl Nat for Zero {}
impl<N: Nat> Nat for Succ<N> {}

//==================================================================================================
// LessThan: Encoding of `<` predicate: `Self < RHS`.

pub trait LessThan<RHS: Nat>: Nat {}

// base case: 0 < (1+n)
impl<RHS: Nat> LessThan<Succ<RHS>> for Zero {}

// inductive case: 1+x < 1+y if x < y
impl<LHS: Nat, RHS: Nat> LessThan<Succ<RHS>> for Succ<LHS>
where LHS: LessThan<RHS> {}


#[cfg(test)]
mod less_than_tests {
    use super::*;

    type One = Succ<Zero>;
    type Two = Succ<One>;

    fn is_less_than_two<T: LessThan<Two>>() {}

    #[test]
    fn test() {
        is_less_than_two::<Zero>(); // 0 < 2
        is_less_than_two::<One>(); // 1 < 2
        // is_less_than_two::<Two>(); // 2 < 2 => error
    }
}
