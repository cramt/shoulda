use crate::Should;
use std::fmt::Debug;

impl<'a, T: Debug> Should<'a, Option<T>> {
    pub fn none(self){
        assert!(
            self.inner.is_none(),
            "{:?} is Some, None expected",
            &self.inner,
        )
    }
    pub fn some(self) {
        assert!(
            self.inner.is_some(),
            "{:?} is None, Some expected",
            &self.inner,
        )
    }
}
