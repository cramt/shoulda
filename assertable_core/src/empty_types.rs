use std::marker::PhantomData;
use crate::Assertable;

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
