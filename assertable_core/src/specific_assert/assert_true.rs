use std::fmt::Debug;

pub trait AssertTrue: Debug {
    fn test_true(&self) -> bool;
    fn assert_true(&self) {
        assert!(
            Self::test_true(self),
            "{:?} is false, true was expected",
            self
        )
    }
}

impl AssertTrue for bool {
    fn test_true(&self) -> bool {
        *self
    }
}
