use crate::Assertable;
use std::marker::PhantomData;

impl Assertable for () {
    fn test_eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T> Assertable for PhantomData<T> {
    fn test_eq(&self, _: &Self) -> bool {
        true
    }
}
