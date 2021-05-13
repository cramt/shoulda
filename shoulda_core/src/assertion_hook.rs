pub trait AssertionHook {
    fn run(input: bool) -> bool;
    fn message_prefix() -> &'static str;
}

pub struct NoOpAssertionHook;

impl AssertionHook for NoOpAssertionHook {
    fn run(input: bool) -> bool {
        input
    }

    fn message_prefix() -> &'static str {
        ""
    }
}

pub struct NotAssertionHook;

impl AssertionHook for NotAssertionHook {
    fn run(input: bool) -> bool {
        !input
    }

    fn message_prefix() -> &'static str {
        "Not: "
    }
}
