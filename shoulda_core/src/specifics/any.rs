use crate::shoulda_of_type::ShouldaOfType;
use crate::Should;
use std::any::{type_name, Any};

impl<'a, T: ShouldaOfType> Should<'a, T> {
    pub fn of_type<I: Any + 'static>(mut self) -> Self {
        self.internal_assert(
            self.inner.should_type_of::<I>(),
            format!(
                "{} is not of the type of {}",
                T::should_name(),
                type_name::<I>()
            ),
        );
        self
    }
}
