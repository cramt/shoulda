pub mod array_like;
pub mod empty_types;
pub mod specifics;
#[cfg(test)]
mod tests;
pub mod wrapper_types;

use once_cell::sync::Lazy;
use std::fmt::Debug;
use std::ops::Deref;
use std::borrow::Borrow;

pub struct Should<'a, T> {
    inner: &'a T,
}

impl<'a, T> Should<'a, T> where T: Shoulda {
    pub fn eq<K: Borrow<T>>(self, other: K) -> Self {
        let other = other.borrow();
        assert!(
            self.inner.test_eq(other),
            "a = {:?}, b = {:?}",
            &self.inner,
            other
        );
        self
    }
    pub fn equal<K: Borrow<T>>(self, other: K) -> Self {
        self.eq(other)
    }
    pub fn not(self) -> ShouldNot<'a, T> {
        ShouldNot {
            inner: self.inner
        }
    }
    pub fn be(self) -> Self {
        self
    }
}

pub struct ShouldNot<'a, T> {
    inner: &'a T,
}

impl<'a, T> ShouldNot<'a, T> where T: Shoulda {
    pub fn eq<K: Borrow<T>>(self, other: K) -> Should<'a, T> {
        let other = other.borrow();
        assert!(
            !self.inner.test_eq(other),
            "a = {:?}, b = {:?}",
            &self.inner,
            other
        );
        self.not()
    }
    pub fn equal<K: Borrow<T>>(self, other: K) -> Should<'a, T> {
        self.eq(other)
    }
    pub fn not(self) -> Should<'a, T> {
        Should {
            inner: self.inner
        }
    }
    pub fn be(self) -> Self {
        self
    }
}

pub trait Shoulda: Debug {
    fn test_eq(&self, other: &Self) -> bool;
    fn should(&self) -> Should<Self> where Self: Sized {
        Should {
            inner: self
        }
    }
}

macro_rules! eq_assertable_impl {
    ($x:ty) => {
        impl Shoulda for $x {
            fn test_eq(&self, other: &Self) -> bool {
                self.eq(other)
            }
        }
    };
}

static SHOULDA_FLOAT_DIFF_MODE: Lazy<f64> = Lazy::new(|| {
    option_env!("SHOULDA_FLOAT_DIFF_MODE")
        .map(|x| x.parse().unwrap())
        .unwrap_or(0.0001)
});

macro_rules! float_assertable_impl {
    ($x:ty) => {
        impl Shoulda for $x {
            fn test_eq(&self, other: &Self) -> bool {
                (self - other).abs() < (*SHOULDA_FLOAT_DIFF_MODE.deref() as $x)
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

impl<T> Shoulda for &T
    where
        T: Shoulda,
{
    fn test_eq(&self, other: &Self) -> bool {
        T::test_eq(self, other)
    }
}
