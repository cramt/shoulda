use crate::float_diff_provider::FloatDiffProvider;
use crate::ShouldaEqual;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Range, RangeInclusive};

macro_rules! zip_all_test_eq_assertable_impl {
    ($x:ty) => {
        impl<T> ShouldaEqual for $x
        where
            T: Debug,
            T: ShouldaEqual,
        {
            fn should_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
                self.iter()
                    .zip(other.iter())
                    .all(|(a, b)| a.should_eq::<FloatDiff>(b))
            }
        }
    };
}

zip_all_test_eq_assertable_impl!(Vec<T>);
zip_all_test_eq_assertable_impl!(VecDeque<T>);
zip_all_test_eq_assertable_impl!(&[T]);
zip_all_test_eq_assertable_impl!([T]);

impl<T, K> ShouldaEqual for HashMap<T, K>
where
    T: Debug,
    T: ShouldaEqual,
    T: Eq,
    T: Hash,
    K: Debug,
    K: ShouldaEqual,
{
    fn should_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self.keys().all(|x| {
                other
                    .get(x)
                    .map(|v| v.should_eq::<FloatDiff>(&self[x]))
                    .unwrap_or(false)
            })
    }
}

impl<T> ShouldaEqual for HashSet<T>
where
    T: Debug,
    T: ShouldaEqual,
    T: Eq,
    T: Hash,
{
    fn should_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        //TODO: make not dependant on Eq
        self.iter().all(|x| other.contains(x))
    }
}

impl<T> ShouldaEqual for Range<T>
where
    T: Debug,
    T: ShouldaEqual,
{
    fn should_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        self.start.should_eq::<FloatDiff>(&other.start)
            && self.end.should_eq::<FloatDiff>(&other.end)
    }
}

impl<T> ShouldaEqual for RangeInclusive<T>
where
    T: Debug,
    T: ShouldaEqual,
{
    fn should_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        self.start().should_eq::<FloatDiff>(other.start())
            && self.end().should_eq::<FloatDiff>(other.end())
    }
}
