pub mod array_like;
pub mod contains;
pub mod empty_types;
#[cfg(test)]
mod tests;
pub mod wrapper_types;

use once_cell::sync::Lazy;
use std::fmt::Debug;
use std::ops::Deref;

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
eq_assertable_impl!(str);
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

impl<T> Assertable for &T
where
    T: Assertable,
{
    fn test_eq(&self, other: &Self) -> bool {
        T::test_eq(self, other)
    }
}
