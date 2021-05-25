use crate::epsilon_provider::EpsilonProvider;
use crate::ShouldaEqual;
use std::borrow::{Borrow, Cow};
use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use std::num::Wrapping;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

impl<K> ShouldaEqual for Option<K>
where
    K: ShouldaEqual,
    K: Debug,
{
    fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool {
        match (self, other) {
            (None, None) => true,
            (Some(a), Some(b)) => a.should_eq::<Epsilon>(b),
            _ => false,
        }
    }
}

impl<L, K> ShouldaEqual for Result<L, K>
where
    L: ShouldaEqual,
    L: Debug,
    K: ShouldaEqual,
    K: Debug,
{
    fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool {
        match (self, other) {
            (Err(a), Err(b)) => a.should_eq::<Epsilon>(b),
            (Ok(a), Ok(b)) => a.should_eq::<Epsilon>(b),
            _ => false,
        }
    }
}

impl<K> ShouldaEqual for Cow<'_, K>
where
    K: ShouldaEqual,
    K: Debug,
    K: ToOwned,
    <K as ToOwned>::Owned: Debug,
    K: ?Sized,
{
    fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool {
        self.deref().should_eq::<Epsilon>(other.deref())
    }
}

impl<T> ShouldaEqual for Box<T>
where
    T: ShouldaEqual,
    T: ?Sized,
{
    fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool {
        self.deref().should_eq::<Epsilon>(other.deref())
    }
}

impl<T> ShouldaEqual for Cell<T>
where
    T: ShouldaEqual,
    T: ?Sized,
    T: Debug,
    T: Copy,
{
    fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool {
        self.get().should_eq::<Epsilon>(&other.borrow().get())
    }
}

impl<T> ShouldaEqual for RefCell<T>
where
    T: ShouldaEqual,
    T: ?Sized,
    T: Debug,
{
    fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool {
        self.borrow().should_eq::<Epsilon>(&*other.borrow())
    }
}

impl<T> ShouldaEqual for Wrapping<T>
where
    T: ShouldaEqual,
    T: Debug,
{
    fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool {
        self.0.should_eq::<Epsilon>(&other.0)
    }
}

impl<T> ShouldaEqual for Rc<T>
where
    T: ShouldaEqual,
    T: ?Sized,
    T: Debug,
{
    fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool {
        (**self).should_eq::<Epsilon>(&**other)
    }
}

impl<T> ShouldaEqual for Arc<T>
where
    T: ShouldaEqual,
    T: ?Sized,
    T: Debug,
{
    fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool {
        (**self).should_eq::<Epsilon>(&**other)
    }
}
