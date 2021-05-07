use std::fmt::Debug;

pub trait AssertNone: Debug {
    fn test_none(&self) -> bool;
    fn assert_none(&self) {
        assert!(
            Self::test_none(self),
            "{:?} is Some, None was expected",
            self
        )
    }
}

impl<T> AssertNone for Option<T>
where
    T: Debug,
{
    fn test_none(&self) -> bool {
        self.is_none()
    }
}
