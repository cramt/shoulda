use crate::{Should, Shoulda, ShouldNot};
use std::borrow::Borrow;
use std::fmt::Debug;

impl<'a, T, K> Should<'a, T>
    where
        &'a T: IntoIterator<Item=&'a K>,
        T: Debug,
        K: Debug,
        K: Shoulda,
        K: 'a,
{
    pub fn contain<I: Borrow<K>>(self, item: I) {
        let item = item.borrow();
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            v.iter().any(|x|x.test_eq(&item)),
            "{:?} did not contain {:?}",
            v,
            item,
        )
    }

    pub fn contains<I: Fn(&K) -> bool>(self, predicate: I) {
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            v.iter().map(|x|*x).any(predicate),
            "{:?} did not fufill the predicate",
            v
        )
    }
}

impl<'a, T, K> ShouldNot<'a, T>
    where
        &'a T: IntoIterator<Item=&'a K>,
        T: Debug,
        K: Debug,
        K: Shoulda,
        K: 'a,
{
    pub fn contain<I: Borrow<K>>(self, item: I) {
        let item = item.borrow();
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            !v.iter().any(|x|x.test_eq(&item)),
            "{:?} did contain {:?}",
            v,
            item,
        )
    }

    pub fn contains<I: Fn(&K) -> bool>(self, predicate: I) {
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            !v.iter().map(|x|*x).any(predicate),
            "{:?} did fufill the predicate",
            v
        )
    }
}
