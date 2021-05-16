use crate::float_diff_provider::FloatDiffProvider;
use crate::Shoulda;
use std::borrow::{Cow, Borrow};
use std::fmt::Debug;
use std::ops::Deref;
use std::cell::{Cell, RefCell};
use std::num::Wrapping;
use std::rc::Rc;
use std::sync::Arc;

impl<K> Shoulda for Option<K>
    where
        K: Shoulda,
        K: Debug,
{
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        match (self, other) {
            (None, None) => true,
            (Some(a), Some(b)) => a.test_eq::<FloatDiff>(b),
            _ => false,
        }
    }
}

impl<L, K> Shoulda for Result<L, K>
    where
        L: Shoulda,
        L: Debug,
        K: Shoulda,
        K: Debug,
{
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        match (self, other) {
            (Err(a), Err(b)) => a.test_eq::<FloatDiff>(b),
            (Ok(a), Ok(b)) => a.test_eq::<FloatDiff>(b),
            _ => false,
        }
    }
}

impl<K> Shoulda for Cow<'_, K>
    where
        K: Shoulda,
        K: Debug,
        K: ToOwned,
        <K as ToOwned>::Owned: Debug,
        K: ?Sized,
{
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        self.deref().test_eq::<FloatDiff>(other.deref())
    }
}

impl<T> Shoulda for Box<T>
    where
        T: Shoulda,
        T: ?Sized,
{
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        self.deref().test_eq::<FloatDiff>(other.deref())
    }
}

impl<T> Shoulda for Cell<T>
    where
        T: Shoulda,
        T: ?Sized,
        T: Debug,
        T: Copy
{
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        self.get().test_eq::<FloatDiff>(&other.borrow().get())
    }
}

impl<T> Shoulda for RefCell<T>
    where
        T: Shoulda,
        T: ?Sized,
        T: Debug,
{
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        self.borrow().test_eq::<FloatDiff>(&*other.borrow())
    }
}

impl<T> Shoulda for Wrapping<T>
    where
        T: Shoulda,
        T: Debug,
{
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        self.0.test_eq::<FloatDiff>(&other.0)
    }
}

impl<T> Shoulda for Rc<T>
    where
        T: Shoulda,
        T: ?Sized,
        T: Debug,
{
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        (**self).test_eq::<FloatDiff>(&**other)
    }
}

impl<T> Shoulda for Arc<T>
    where
        T: Shoulda,
        T: ?Sized,
        T: Debug,
{
    fn test_eq<FloatDiff: FloatDiffProvider>(&self, other: &Self) -> bool {
        (**self).test_eq::<FloatDiff>(&**other)
    }
}
