use std::panic::{UnwindSafe, RefUnwindSafe};
use std::any::Any;
use std::marker::PhantomData;
use crate::{Should, Shoulda};
use crate::assertion_hook::AssertionHook;
use crate::float_diff_provider::FloatDiffProvider;
use std::fmt::Debug;

pub struct Expression<T: RefUnwindSafe, F: FnOnce(&T) + UnwindSafe> {
    func: F,
    source: String,
    _a: PhantomData<T>,
}

impl<T: RefUnwindSafe, F: FnOnce(&T) + UnwindSafe> Expression<T, F> {
    pub fn new(func: F, source: String) -> Self {
        Self {func, source, _a: Default::default()}
    }
}

pub trait Panicable<'a, T: Debug> {
    fn run(self, inner: &'a T) -> (Option<Box<dyn Any + Send + 'static>>, String);
}

impl<'a, T: RefUnwindSafe + Debug, F: FnOnce(&T) + UnwindSafe> Panicable<'a, T> for Expression<T, F> {
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

impl<'a, Inner, Hook, FloatDiff> Should<'a, Inner, Hook, FloatDiff>
    where
        Inner: Debug,
        Inner: RefUnwindSafe,
        Hook: AssertionHook,
        FloatDiff: FloatDiffProvider,
{
    pub fn panic<Expr: Panicable<'a, Inner>>(mut self, e: Expr) -> Self {
        let (result, message) = e.run(&self.inner);
        self.internal_assert(result.is_some(), format!("{} didnt panic", message));
        self
    }
    pub fn panic_with<Expr: Panicable<'a, Inner>, S: AsRef<str>>(mut self, e: Expr, message: S) -> Self {
        let (result, assert_message) = e.run(&self.inner);
        self.internal_assert(result.is_some(), format!("{} didnt panic", assert_message));
        match result {
            None => {}
            Some(x) => {
                let actual_message = x.downcast_ref::<String>().unwrap().as_str();
                self.internal_assert(actual_message.test_eq::<FloatDiff>(message.as_ref()), format!("{} paniced with output {}, {} was expected", assert_message, actual_message, message.as_ref()))
            }
        };
        self
    }
}
