use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::hash::Hash;

pub trait Assertable: Debug {
    fn test_eq(&self, other: &Self) -> bool;
    fn assert_eq(&self, other: &Self) {
        assert!(
            Self::test_eq(self, other),
            "a = {:?}, b = {:?}",
            self,
            other
        )
    }
    fn assert_ne(&self, other: &Self) {
        assert!(
            !Self::test_eq(self, other),
            "a = {:?}, b = {:?}",
            self,
            other
        )
    }
}

macro_rules! eq_assertable_impl {
    ($x:ty) => {
        impl Assertable for $x {
            fn test_eq(&self, other: &Self) -> bool {
                self.eq(other)
            }
        }
    };
}

static ASSERTABLE_FLOAT_DIFF_MODE: Lazy<f64> = Lazy::new(|| {
    option_env!("ASSERTABLE_FLOAT_DIFF_MODE")
        .map(|x| x.parse().unwrap())
        .unwrap_or(0.0001)
});

macro_rules! float_assertable_impl {
    ($x:ty) => {
        impl Assertable for $x {
            fn test_eq(&self, other: &Self) -> bool {
                (self - other).abs() < (*ASSERTABLE_FLOAT_DIFF_MODE.deref() as $x)
            }
        }
    };
}

eq_assertable_impl!(String);
eq_assertable_impl!(&str);
eq_assertable_impl!(bool);
eq_assertable_impl!(u8);
eq_assertable_impl!(i8);
eq_assertable_impl!(u16);
eq_assertable_impl!(i16);
eq_assertable_impl!(u32);
eq_assertable_impl!(i32);
eq_assertable_impl!(u64);
eq_assertable_impl!(i64);
eq_assertable_impl!(u128);
eq_assertable_impl!(i128);
eq_assertable_impl!(usize);
eq_assertable_impl!(isize);

float_assertable_impl!(f32);
float_assertable_impl!(f64);

impl<K> Assertable for Option<K>
where
    K: Assertable,
    K: Debug,
{
    fn test_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (None, None) => true,
            (Some(a), Some(b)) => a.test_eq(b),
            _ => false,
        }
    }
}

impl<L, K> Assertable for Result<L, K>
where
    L: Assertable,
    L: Debug,
    K: Assertable,
    K: Debug,
{
    fn test_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Err(a), Err(b)) => a.test_eq(b),
            (Ok(a), Ok(b)) => a.test_eq(b),
            _ => false,
        }
    }
}

impl<K> Assertable for Cow<'_, K>
where
    K: Assertable,
    K: Debug,
    K: Clone,
{
    fn test_eq(&self, other: &Self) -> bool {
        self.deref().test_eq(other.deref())
    }
}

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

impl<T> Assertable for Vec<T>
where
    T: Debug,
    T: Assertable,
{
    fn test_eq(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(a, b)| a.test_eq(b))
    }
}

impl<T> Assertable for VecDeque<T>
where
    T: Debug,
    T: Assertable,
{
    fn test_eq(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(a, b)| a.test_eq(b))
    }
}

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
