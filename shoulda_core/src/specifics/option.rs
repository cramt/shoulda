use crate::Should;
use std::fmt::Debug;

impl<'a, T: Debug> Should<'a, Option<T>> {
    pub fn none(self) -> Self {
        assert!(
            self.inner.is_none(),
            "{:?} is Some, None expected",
            &self.inner,
        );
        self
    }
    pub fn some(self) -> Should<'a, T> {
        assert!(
            self.inner.is_some(),
            "{:?} is None, Some expected",
            &self.inner,
        );
        Should {
            inner: self.inner.as_ref().unwrap(),
        }
    }
}
