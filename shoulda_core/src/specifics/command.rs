use std::process::Output;

use crate::{assertion_hook::NoOpAssertionHook, Should};

impl<'a> Should<'a, Output> {
    pub fn succeed(mut self) -> Should<'a, Output, NoOpAssertionHook> {
        self.internal_assert(
            self.inner.status.success(),
            "command was not a success".to_string(),
        );
        self.normalize()
    }

    pub fn code(mut self, expected: i32) -> Should<'a, Output, NoOpAssertionHook> {
        let actual = self.inner.status.code();
        self.internal_assert(
            actual == Some(expected),
            format!("{:?} did not equal {}", actual, expected),
        );
        self.normalize()
    }

    pub fn stdout(mut self, bytes: &[u8]) -> Should<'a, Output, NoOpAssertionHook> {
        self.internal_assert(
            self.inner.stdout.as_slice() == bytes,
            format!("{:?} did not equal {:?}", self.inner.stdout, bytes),
        );
        self.normalize()
    }

    pub fn stderr(mut self, bytes: &[u8]) -> Should<'a, Output, NoOpAssertionHook> {
        self.internal_assert(
            self.inner.stderr.as_slice() == bytes,
            format!("{:?} did not equal {:?}", self.inner.stdout, bytes),
        );
        self.normalize()
    }
}
