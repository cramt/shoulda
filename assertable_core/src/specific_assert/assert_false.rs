use std::fmt::Debug;

pub trait AssertFalse: Debug {
    fn test_false(&self) -> bool;
    fn assert_false(&self) {
        assert!(
            Self::test_false(self),
            "{:?} is true, false was expected",
            self
        )
    }
}

impl AssertFalse for bool {
    fn test_false(&self) -> bool {
        !*self
    }
}
