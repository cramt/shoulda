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
    pub fn contain<I: Borrow<K>>(self, item: I) -> Self {
        let item = item.borrow();
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            v.iter().any(|x|x.test_eq(&item)),
            "{:?} did not contain {:?}",
            v,
            item,
        );
        self
    }

    pub fn contains<I: Fn(&K) -> bool>(self, predicate: I) -> Self {
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            v.iter().map(|x|*x).any(predicate),
            "{:?} did not fufill the predicate",
            v
        );
        self
    }
}

impl<'a, T, K> ShouldNot<'a, T>
    where
        &'a T: IntoIterator<Item=&'a K>,
        T: Debug,
        T: Shoulda,
        K: Debug,
        K: Shoulda,
        K: 'a,
{
    pub fn contain<I: Borrow<K>>(self, item: I) -> Should<'a, T> {
        let item = item.borrow();
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            !v.iter().any(|x|x.test_eq(&item)),
            "{:?} did contain {:?}",
            v,
            item,
        );
        self.not()
    }

    pub fn contains<I: Fn(&K) -> bool>(self, predicate: I) -> Should<'a, T> {
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            !v.iter().map(|x|*x).any(predicate),
            "{:?} did fufill the predicate",
            v
        );
        self.not()
    }
}
