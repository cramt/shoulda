use crate::Assertable;

pub trait AssertableContains<Inner: Assertable>: Assertable {
    fn test_contains(&self, inner: &Inner) -> bool;
    fn assert_contains(&self, inner: &Inner) {
        assert!(self.test_contains(inner), "TODO some debug info")
    }
}
