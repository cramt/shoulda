pub mod array_like;
pub mod assertion_hook;
pub mod empty_types;
pub mod float_diff_provider;
pub mod should_result;
pub mod specifics;
#[cfg(test)]
mod tests;
pub mod wrapper_types;

use crate::assertion_hook::{AssertionHook, NoOpAssertionHook, NotAssertionHook, OrAssertionHook};
use crate::float_diff_provider::{EnvFloatDiffProvider, FloatDiffProvider};
use crate::should_result::ResultsContainer;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::DerefMut;

pub struct Should<
    'a,
    Inner,
    Hook: AssertionHook = NoOpAssertionHook,
    FloatDiff: FloatDiffProvider = EnvFloatDiffProvider,
> {
    results: ResultsContainer,
    inner: &'a Inner,
    hook: PhantomData<Hook>,
    float_diff: PhantomData<FloatDiff>,
}

impl<'a, Inner, Hook, FloatDiff> Should<'a, Inner, Hook, FloatDiff>
where
    Hook: AssertionHook,
    FloatDiff: FloatDiffProvider,
{
    pub(crate) fn internal_assert(&mut self, initial: bool, message: String) {
        Hook::create_result(initial, message, self.results.deref_mut())
    }
    pub(crate) fn change_optional_generics<T: AssertionHook, L: FloatDiffProvider>(
        self,
    ) -> Should<'a, Inner, T, L> {
        Should::new(self.inner, self.results)
    }
    pub(crate) fn normalize(self) -> Should<'a, Inner, NoOpAssertionHook, FloatDiff> {
        self.change_optional_generics()
    }
    pub(crate) fn new(inner: &'a Inner, results: ResultsContainer) -> Self {
        Self {
            results,
            inner,
            hook: Default::default(),
            float_diff: Default::default(),
        }
    }
}

impl<'a, Inner, Hook, FloatDiff> Should<'a, Inner, Hook, FloatDiff>
where
    Inner: Shoulda,
    Hook: AssertionHook,
    FloatDiff: FloatDiffProvider,
{
    pub fn eq<K: Borrow<Inner>>(mut self, other: K) -> Self {
        let other = other.borrow();
        self.internal_assert(
            self.inner.test_eq::<FloatDiff>(other),
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
    pub fn float_diff<T: FloatDiffProvider>(self) -> Should<'a, Inner, Hook, T> {
        self.change_optional_generics()
    }
}

impl<'a, Inner, FloatDiff> Should<'a, Inner, NoOpAssertionHook, FloatDiff>
where
    Inner: Shoulda,
    FloatDiff: FloatDiffProvider,
{
    pub fn not(self) -> Should<'a, Inner, NotAssertionHook, FloatDiff> {
        self.change_optional_generics()
    }
}

impl<'a, Inner, FloatDiff> Should<'a, Inner, NoOpAssertionHook, FloatDiff>
where
    Inner: Shoulda,
    FloatDiff: FloatDiffProvider,
{
    pub fn or(self) -> Should<'a, Inner, OrAssertionHook, FloatDiff> {
        self.change_optional_generics()
    }
}

impl<'a, Inner, FloatDiff> Should<'a, Inner, NotAssertionHook, FloatDiff>
where
    Inner: Shoulda,
    FloatDiff: FloatDiffProvider,
{
    pub fn not(self) -> Should<'a, Inner, NoOpAssertionHook, FloatDiff> {
        self.change_optional_generics()
    }
}

pub trait Shoulda: Debug {
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool;
    fn should(&self) -> Should<Self>
    where
        Self: Sized,
    {
        Should::new(self, ResultsContainer::default())
    }
}

macro_rules! eq_assertable_impl {
    ($x:ty) => {
        impl Shoulda for $x {
            fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
                self.eq(other)
            }
        }
    };
}

macro_rules! float_assertable_impl {
    ($x:ty) => {
        impl Shoulda for $x {
            fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
                (self - other).abs() < (FloatDiff::diff() as $x)
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
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        T::test_eq::<FloatDiff>(self, other)
    }
}
