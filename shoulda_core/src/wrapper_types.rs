use crate::Shoulda;
use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::Deref;

impl<K> Shoulda for Option<K>
where
    K: Shoulda,
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

impl<L, K> Shoulda for Result<L, K>
where
    L: Shoulda,
    L: Debug,
    K: Shoulda,
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

impl<K> Shoulda for Cow<'_, K>
where
    K: Shoulda,
    K: Debug,
    K: ToOwned,
    <K as ToOwned>::Owned: Debug,
    K: ?Sized,
{
    fn test_eq(&self, other: &Self) -> bool {
        self.deref().test_eq(other.deref())
    }
}

impl<T> Shoulda for Box<T>
where
    T: Shoulda,
    T: ?Sized,
{
    fn test_eq(&self, other: &Self) -> bool {
        self.deref().test_eq(other.deref())
    }
}
