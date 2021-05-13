pub mod array_like;
pub mod assertion_hook;
pub mod empty_types;
pub mod specifics;
#[cfg(test)]
mod tests;
pub mod wrapper_types;

use crate::assertion_hook::{AssertionHook, NoOpAssertionHook, NotAssertionHook};
use once_cell::sync::Lazy;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

pub struct Should<'a, Inner, Hook: AssertionHook = NoOpAssertionHook> {
    inner: &'a Inner,
    hook: PhantomData<Hook>,
}

impl<'a, Inner, Hook> Should<'a, Inner, Hook>
where
    Hook: AssertionHook,
{
    pub(crate) fn internal_assert(&self, initial: bool, message: String) {
        assert!(Hook::run(initial), "{}{}", Hook::message_prefix(), message);
    }
    pub(crate) fn change_hook<T: AssertionHook>(self) -> Should<'a, Inner, T> {
        Should::new(self.inner)
    }
    pub(crate) fn normalize(self) -> Should<'a, Inner, NoOpAssertionHook> {
        self.change_hook()
    }
    pub(crate) fn new(inner: &'a Inner) -> Self {
        Self {
            inner,
            hook: Default::default(),
        }
    }
}

impl<'a, Inner, Hook> Should<'a, Inner, Hook>
where
    Inner: Shoulda,
    Hook: AssertionHook,
{
    pub fn eq<K: Borrow<Inner>>(self, other: K) -> Self {
        let other = other.borrow();
        self.internal_assert(
            self.inner.test_eq(other),
            format!("expected = {:?}, actual = {:?}", &self.inner, other),
        );
        self
    }
    pub fn equal<K: Borrow<Inner>>(self, other: K) -> Self {
        self.eq(other)
    }
    pub fn be(self) -> Self {
        self
    }
    pub fn and(self) -> Self {
        self
    }
}

impl<'a, Inner> Should<'a, Inner, NoOpAssertionHook>
where
    Inner: Shoulda,
{
    pub fn not(self) -> Should<'a, Inner, NotAssertionHook> {
        self.change_hook()
    }
}

impl<'a, Inner> Should<'a, Inner, NotAssertionHook>
where
    Inner: Shoulda,
{
    pub fn not(self) -> Should<'a, Inner, NoOpAssertionHook> {
        self.change_hook()
    }
}

pub trait Shoulda: Debug {
    fn test_eq(&self, other: &Self) -> bool;
    fn should(&self) -> Should<Self>
    where
        Self: Sized,
    {
        Should::new(self)
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
