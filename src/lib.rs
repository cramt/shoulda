use crate::core::Should;
pub use shoulda_core as core;
pub use shoulda_core::Shoulda;
pub use shoulda_macro::expr as expr_internal;
pub use shoulda_macro::Shoulda;

#[macro_export]
macro_rules! expr {
    ($x:expr) => {
        ::shoulda::expr_internal!($x)
    };
}

#[macro_export]
macro_rules! expect {
    ($x:tt) => {
        ::shoulda::Expect::new(&$x)
    };
}

pub struct Expect<'a, T: Shoulda> {
    inner: &'a T,
}

impl<'a, T: Shoulda> Expect<'a, T> {
    pub fn new(inner: &'a T) -> Self {
        Self { inner }
    }
    pub fn to(self) -> Should<'a, T> {
        self.inner.should()
    }
}
