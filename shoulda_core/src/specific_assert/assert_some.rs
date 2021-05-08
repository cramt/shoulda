use std::fmt::Debug;

pub trait AssertSome: Debug {
    fn test_some(&self) -> bool;
    fn assert_some(&self) {
        assert!(
            Self::test_some(self),
            "{:?} is None, Some was expected",
            self
        )
    }
}

impl<T> AssertSome for Option<T>
where
    T: Debug,
{
    fn test_some(&self) -> bool {
        self.is_some()
    }
}
