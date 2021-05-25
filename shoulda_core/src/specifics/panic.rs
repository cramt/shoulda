use crate::assertion_hook::AssertionHook;
use crate::epsilon_provider::EpsilonProvider;
use crate::shoulda_equal::ShouldaEqual;
use crate::Should;
use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::panic::{RefUnwindSafe, UnwindSafe};

pub struct Expression<T: RefUnwindSafe, F: FnOnce(&T) + UnwindSafe> {
    func: F,
    source: String,
    _a: PhantomData<T>,
}

impl<T: RefUnwindSafe, F: FnOnce(&T) + UnwindSafe> Expression<T, F> {
    pub fn new(func: F, source: String) -> Self {
        Self {
            func,
            source,
            _a: Default::default(),
        }
    }
}

pub trait Panicable<'a, T: Debug> {
    fn run(self, inner: &'a T) -> (Option<Box<dyn Any + Send + 'static>>, String);
}

impl<'a, T: RefUnwindSafe + Debug, F: FnOnce(&T) + UnwindSafe> Panicable<'a, T>
    for Expression<T, F>
{
    fn run(self, inner: &'a T) -> (Option<Box<dyn Any + Send>>, String) {
        let (result, _) = self.func.run(inner);
        (result, self.source)
    }
}

impl<'a, T: RefUnwindSafe + Debug, F: FnOnce(&T) + UnwindSafe> Panicable<'a, T> for F {
    fn run(self, inner: &'a T) -> (Option<Box<dyn Any + Send>>, String) {
        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let result = std::panic::catch_unwind(|| self(inner));
        std::panic::set_hook(previous_hook);
        (result.err(), String::from("anonymous closure"))
    }
}

impl<'a, Inner, Hook, Epsilon> Should<'a, Inner, Hook, Epsilon>
where
    Inner: Debug,
    Inner: RefUnwindSafe,
    Hook: AssertionHook,
    Epsilon: EpsilonProvider,
{
    pub fn panic<Expr: Panicable<'a, Inner>>(mut self, e: Expr) -> Self {
        let (result, message) = e.run(&self.inner);
        self.internal_assert(result.is_some(), format!("{} didnt panic", message));
        self
    }
    pub fn panic_with<Expr: Panicable<'a, Inner>, S: AsRef<str>>(
        mut self,
        e: Expr,
        message: S,
    ) -> Self {
        let (result, assert_message) = e.run(&self.inner);
        self.internal_assert(result.is_some(), format!("{} didnt panic", assert_message));
        match result {
            None => {}
            Some(x) => {
                let actual_message = x.downcast_ref::<String>().unwrap().as_str();
                self.internal_assert(
                    actual_message.should_eq::<Epsilon>(message.as_ref()),
                    format!(
                        "{} paniced with output {}, {} was expected",
                        assert_message,
                        actual_message,
                        message.as_ref()
                    ),
                )
            }
        };
        self
    }
}
