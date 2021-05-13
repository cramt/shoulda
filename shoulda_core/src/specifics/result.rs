use crate::Should;
use std::fmt::Debug;

impl<'a, T: Debug, E: Debug> Should<'a, Result<T, E>> {
    pub fn err(self) -> Should<'a, E> {
        self.internal_assert(
            self.inner.is_err(),
            format!("{:?} is Ok, Err expected", &self.inner),
        );
        Should::new(self.inner.as_ref().err().unwrap())
    }
    pub fn ok(self) -> Should<'a, T> {
        self.internal_assert(
            self.inner.is_ok(),
            format!("{:?} is Err, Ok expected", &self.inner),
        );
        Should::new(self.inner.as_ref().unwrap())
    }
}
