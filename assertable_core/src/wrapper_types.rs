use crate::Assertable;
use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::Deref;

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
