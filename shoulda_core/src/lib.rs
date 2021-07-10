pub mod array_like;
pub mod assertion_hook;
pub mod empty_types;
pub mod epsilon_provider;
pub mod should_result;
pub mod shoulda_equal;
pub mod shoulda_of_type;
pub mod specifics;
#[cfg(test)]
mod tests;
pub mod tuples;
pub mod wrapper_types;

use crate::assertion_hook::{AssertionHook, NoOpAssertionHook, NotAssertionHook, OrAssertionHook};
use crate::epsilon_provider::{EnvEpsilonProvider, EpsilonProvider};
use crate::should_result::ResultsContainer;
use crate::shoulda_equal::ShouldaEqual;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::DerefMut;

/// A wrapper type around a borrowed value to assert things about it for testing
pub struct Should<
    'a,
    Inner,
    Hook: AssertionHook = NoOpAssertionHook,
    Epsilon: EpsilonProvider = EnvEpsilonProvider,
> {
    results: ResultsContainer,
    inner: &'a Inner,
    hook: PhantomData<Hook>,
    float_diff: PhantomData<Epsilon>,
}

impl<'a, Inner, Hook, Epsilon> Should<'a, Inner, Hook, Epsilon>
where
    Hook: AssertionHook,
    Epsilon: EpsilonProvider,
{
    pub(crate) fn internal_assert(&mut self, initial: bool, message: String) {
        Hook::create_result(initial, message, self.results.deref_mut())
    }
    pub(crate) fn change_optional_generics<T: AssertionHook, L: EpsilonProvider>(
        self,
    ) -> Should<'a, Inner, T, L> {
        Should::new(self.inner, self.results)
    }
    pub(crate) fn normalize(self) -> Should<'a, Inner, NoOpAssertionHook, Epsilon> {
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

    /// no-op, there to allow you to write english-like sentences
    pub fn be(self) -> Self {
        self
    }

    /// no-op, there to allow you to write english-like sentences
    pub fn and(self) -> Self {
        self
    }

    /// Changes the generic for which EpsilonProvider to use
    pub fn float_diff<T: EpsilonProvider>(self) -> Should<'a, Inner, Hook, T> {
        self.change_optional_generics()
    }
}

impl<'a, Inner, Hook, Epsilon> Should<'a, Inner, Hook, Epsilon>
where
    Inner: ShouldaEqual + Debug,
    Hook: AssertionHook,
    Epsilon: EpsilonProvider,
{
    /// Asserts equality
    pub fn eq<K: Borrow<Inner>>(mut self, other: K) -> Self {
        let other = other.borrow();
        self.internal_assert(
            self.inner.should_eq::<Epsilon>(other),
            format!("expected = {:?}, actual = {:?}", &self.inner, other),
        );
        self
    }

    /// alias for eq
    pub fn equal<K: Borrow<Inner>>(self, other: K) -> Self {
        self.eq(other)
    }
}

impl<'a, Inner, Epsilon> Should<'a, Inner, NoOpAssertionHook, Epsilon>
where
    Epsilon: EpsilonProvider,
{
    /// Changes the assertion hook to not the next assertion
    pub fn not(self) -> Should<'a, Inner, NotAssertionHook, Epsilon> {
        self.change_optional_generics()
    }
}

impl<'a, Inner, Epsilon> Should<'a, Inner, NoOpAssertionHook, Epsilon>
where
    Epsilon: EpsilonProvider,
{
    /// Changes the assertion hook to or the next assertion with the previous, in case of no
    /// previous assertion this will be a no-op
    pub fn or(self) -> Should<'a, Inner, OrAssertionHook, Epsilon> {
        self.change_optional_generics()
    }
}

impl<'a, Inner, Epsilon> Should<'a, Inner, NotAssertionHook, Epsilon>
where
    Epsilon: EpsilonProvider,
{
    /// Changes the assertion hook to not the next assertion
    pub fn not(self) -> Should<'a, Inner, NoOpAssertionHook, Epsilon> {
        self.change_optional_generics()
    }
}

pub trait Shoulda {
    fn should(&self) -> Should<Self>
    where
        Self: Sized,
    {
        Should::new(self, ResultsContainer::default())
    }
}

impl<T> Shoulda for T {}
