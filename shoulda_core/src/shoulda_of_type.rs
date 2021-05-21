use std::any::{type_name, Any, TypeId};

pub trait ShouldaOfType {
    fn should_type_of<I: Any + 'static>(&self) -> bool;
    fn should_name() -> &'static str;
}

impl<T> ShouldaOfType for T
where
    T: AsRef<dyn Any>,
{
    fn should_type_of<I: Any + 'static>(&self) -> bool {
        self.as_ref().type_id().eq(&TypeId::of::<I>())
    }

    fn should_name() -> &'static str {
        type_name::<T>()
    }
}
