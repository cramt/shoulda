use crate::Assertable;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Range, RangeInclusive};

macro_rules! zip_all_test_eq_assertable_impl {
    ($x:ty) => {
        impl<T> Assertable for $x
        where
            T: Debug,
            T: Assertable,
        {
            fn test_eq(&self, other: &Self) -> bool {
                self.iter().zip(other.iter()).all(|(a, b)| a.test_eq(b))
            }
        }
    };
}

impl<T> Assertable for Vec<T>
    where
        T: Debug,
        T: Assertable,
{
    fn test_eq(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(a, b)| a.test_eq(b))
    }
}

//zip_all_test_eq_assertable_impl!(Vec<T>);
zip_all_test_eq_assertable_impl!(VecDeque<T>);
zip_all_test_eq_assertable_impl!(&[T]);

impl<T, K> Assertable for HashMap<T, K>
    where
        T: Debug,
        T: Assertable,
        T: Eq,
        T: Hash,
        K: Debug,
        K: Assertable,
{
    fn test_eq(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self
            .keys()
            .all(|x| other.get(x).map(|v| v.test_eq(&self[x])).unwrap_or(false))
    }
}

impl<T> Assertable for HashSet<T>
    where
        T: Debug,
        T: Assertable,
        T: Eq,
        T: Hash,
{
    fn test_eq(&self, other: &Self) -> bool {
        self.iter().all(|x| other.contains(x))
    }
}

impl<T> Assertable for Range<T>
    where
        T: Debug,
        T: Assertable,
{
    fn test_eq(&self, other: &Self) -> bool {
        self.start.test_eq(&other.start) && self.end.test_eq(&other.end)
    }
}

impl<T> Assertable for RangeInclusive<T>
    where
        T: Debug,
        T: Assertable,
{
    fn test_eq(&self, other: &Self) -> bool {
        self.start().test_eq(other.start()) && self.end().test_eq(other.end())
    }
}
