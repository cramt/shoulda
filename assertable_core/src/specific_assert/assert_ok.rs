use std::fmt::Debug;

pub trait AssertOk: Debug {
    fn test_ok(&self) -> bool;
    fn assert_ok(&self) {
        assert!(Self::test_ok(self), "{:?} is Err, Ok was expected", self)
    }
}

impl<T, K> AssertOk for Result<T, K>
where
    T: Debug,
    K: Debug,
{
    fn test_ok(&self) -> bool {
        self.is_ok()
    }
}
