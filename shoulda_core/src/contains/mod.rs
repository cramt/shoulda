use crate::Shoulda;

pub trait AssertableContains<Inner: Shoulda>: Shoulda {
    fn test_contains(&self, inner: &Inner) -> bool;
    fn assert_contains(&self, inner: &Inner) {
        assert!(self.test_contains(inner), "TODO some debug info")
    }
}
