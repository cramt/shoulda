use crate::assertion_hook::NoOpAssertionHook;
use crate::Should;
use std::fmt::Debug;

impl<'a, T: Debug> Should<'a, Option<T>> {
    pub fn none(mut self) -> Should<'a, Option<T>, NoOpAssertionHook> {
        self.internal_assert(
            self.inner.is_none(),
            format!("{:?} is Some, None expected", &self.inner,),
        );
        self.normalize()
    }
    pub fn some(mut self) -> Should<'a, Option<T>> {
        self.internal_assert(
            self.inner.is_some(),
            format!("{:?} is None, Some expected", &self.inner,),
        );
        self.normalize()
    }
}
