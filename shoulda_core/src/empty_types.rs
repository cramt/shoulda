use crate::epsilon_provider::EpsilonProvider;
use crate::ShouldaEqual;
use std::marker::PhantomData;

impl ShouldaEqual for () {
    fn should_eq<Epsilon: EpsilonProvider>(&self, _: &Self) -> bool {
        true
    }
}

impl<T> ShouldaEqual for PhantomData<T> {
    fn should_eq<Epsilon: EpsilonProvider>(&self, _: &Self) -> bool {
        true
    }
}
