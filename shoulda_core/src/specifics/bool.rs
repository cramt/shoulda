use crate::Should;

impl<'a> Should<'a, bool> {
    pub fn be_true(self){
        assert!(
            self.inner,
            "{:?} is false, true expected",
            &self.inner,
        )
    }
    pub fn be_false(self){
        assert!(
            !self.inner,
            "{:?} is true, false expected",
            &self.inner,
        )
    }
}
