use crate::float_diff_provider::FloatDiffProvider;
use crate::ShouldaEqual;
use std::marker::PhantomData;

impl ShouldaEqual for () {
    fn should_eq<FloatDiff: FloatDiffProvider>(&self, _: &Self) -> bool {
        true
    }
}

impl<T> ShouldaEqual for PhantomData<T> {
    fn should_eq<FloatDiff: FloatDiffProvider>(&self, _: &Self) -> bool {
        true
    }
}
