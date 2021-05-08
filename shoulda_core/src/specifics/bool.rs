use crate::Should;

impl<'a> Should<'a, bool> {
    pub fn be_true(self) -> Self {
        assert!(self.inner, "{:?} is false, true expected", &self.inner,);
        self
    }
    pub fn be_false(self) -> Self {
        assert!(!self.inner, "{:?} is true, false expected", &self.inner,);
        self
    }
}
