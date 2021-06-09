use crate::assertion_hook::NoOpAssertionHook;
use crate::Should;

impl<'a> Should<'a, bool> {
    pub fn be_true(mut self) -> Should<'a, bool, NoOpAssertionHook> {
        self.internal_assert(
            *self.inner,
            format!("{:?} is false, true expected", &self.inner),
        );
        self.normalize()
    }
    pub fn be_false(mut self) -> Should<'a, bool, NoOpAssertionHook> {
        self.internal_assert(
            !*self.inner,
            format!("{:?} is true, false expected", &self.inner),
        );
        self.normalize()
    }
}
