use crate::Shoulda;
use std::marker::PhantomData;

impl Shoulda for () {
    fn test_eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T> Shoulda for PhantomData<T> {
    fn test_eq(&self, _: &Self) -> bool {
        true
    }
}
