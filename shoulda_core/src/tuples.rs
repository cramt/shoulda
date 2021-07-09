use impl_trait_for_tuples::*;

use crate::{epsilon_provider::EpsilonProvider, shoulda_equal::ShouldaEqual};

#[impl_for_tuples(1, 100)]
impl ShouldaEqual for Tuple {
    fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool {
        let a = [for_tuples!(#( self.Tuple.should_eq::<Epsilon>(&other.Tuple)),*)];
        a.iter().any(|x| *x)
    }
}
