use crate::Should;
use std::fmt::Debug;

impl<'a, T: Debug, E: Debug> Should<'a, Result<T, E>> {
    pub fn err(mut self) -> Should<'a, Result<T, E>> {
        self.internal_assert(
            self.inner.is_err(),
            format!("{:?} is Ok, Err expected", &self.inner),
        );
        self.normalize()
    }
    pub fn ok(mut self) -> Should<'a, Result<T, E>> {
        self.internal_assert(
            self.inner.is_ok(),
            format!("{:?} is Err, Ok expected", &self.inner),
        );
        self.normalize()
    }
}
