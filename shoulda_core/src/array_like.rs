use crate::float_diff_provider::FloatDiffProvider;
use crate::Shoulda;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Range, RangeInclusive};

macro_rules! zip_all_test_eq_assertable_impl {
    ($x:ty) => {
        impl<T> Shoulda for $x
        where
            T: Debug,
            T: Shoulda,
        {
            fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
                self.iter()
                    .zip(other.iter())
                    .all(|(a, b)| a.test_eq::<FloatDiff>(b))
            }
        }
    };
}

zip_all_test_eq_assertable_impl!(Vec<T>);
zip_all_test_eq_assertable_impl!(VecDeque<T>);
zip_all_test_eq_assertable_impl!(&[T]);
zip_all_test_eq_assertable_impl!([T]);

impl<T, K> Shoulda for HashMap<T, K>
where
    T: Debug,
    T: Shoulda,
    T: Eq,
    T: Hash,
    K: Debug,
    K: Shoulda,
{
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self.keys().all(|x| {
                other
                    .get(x)
                    .map(|v| v.test_eq::<FloatDiff>(&self[x]))
                    .unwrap_or(false)
            })
    }
}

impl<T> Shoulda for HashSet<T>
where
    T: Debug,
    T: Shoulda,
    T: Eq,
    T: Hash,
{
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        //TODO: make not dependant on Eq
        self.iter().all(|x| other.contains(x))
    }
}

impl<T> Shoulda for Range<T>
where
    T: Debug,
    T: Shoulda,
{
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        self.start.test_eq::<FloatDiff>(&other.start) && self.end.test_eq::<FloatDiff>(&other.end)
    }
}

impl<T> Shoulda for RangeInclusive<T>
where
    T: Debug,
    T: Shoulda,
{
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        self.start().test_eq::<FloatDiff>(other.start())
            && self.end().test_eq::<FloatDiff>(other.end())
    }
}
