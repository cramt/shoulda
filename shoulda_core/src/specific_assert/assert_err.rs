use std::fmt::Debug;

pub trait AssertErr: Debug {
    fn test_err(&self) -> bool;
    fn assert_err(&self) {
        assert!(Self::test_err(self), "{:?} is Ok, Err was expected", self)
    }
}

impl<T, K> AssertErr for Result<T, K>
where
    T: Debug,
    K: Debug,
{
    fn test_err(&self) -> bool {
        self.is_err()
    }
}
