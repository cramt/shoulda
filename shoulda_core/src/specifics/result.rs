use crate::Should;
use std::fmt::Debug;

impl<'a, T: Debug, E: Debug> Should<'a, Result<T, E>> {
    pub fn err(self) {
        assert!(
            self.inner.is_err(),
            "{:?} is Ok, Err expected",
            &self.inner,
        )
    }
    pub fn ok(self) {
        assert!(
            self.inner.is_ok(),
            "{:?} is Err, Ok expected",
            &self.inner,
        )
    }
}
