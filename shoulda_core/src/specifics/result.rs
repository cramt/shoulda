use crate::Should;
use std::fmt::Debug;

impl<'a, T: Debug, E: Debug> Should<'a, Result<T, E>> {
    pub fn err(self) -> Should<'a, E> {
        assert!(
            self.inner.is_err(),
            "{:?} is Ok, Err expected",
            &self.inner,
        );
        Should {
            inner: self.inner.as_ref().err().unwrap()
        }
    }
    pub fn ok(self) -> Should<'a, T> {
        assert!(
            self.inner.is_ok(),
            "{:?} is Err, Ok expected",
            &self.inner,
        );
        Should {
            inner: self.inner.as_ref().unwrap()
        }
    }
}
