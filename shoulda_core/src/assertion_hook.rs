use crate::should_result::ShouldResult;

pub trait AssertionHook {
    fn create_result(pass: bool, message: String, results: &mut Vec<ShouldResult>);
}

pub struct NoOpAssertionHook;

impl AssertionHook for NoOpAssertionHook {
    fn create_result(pass: bool, message: String, results: &mut Vec<ShouldResult>) {
        results.push((pass, message).into())
    }
}

pub struct NotAssertionHook;

impl AssertionHook for NotAssertionHook {
    fn create_result(pass: bool, message: String, results: &mut Vec<ShouldResult>) {
        results.push((!pass, format!("NOT: {}", message)).into())
    }
}

pub struct OrAssertionHook;

impl AssertionHook for OrAssertionHook {
    fn create_result(pass: bool, message: String, results: &mut Vec<ShouldResult>) {
        match results.pop() {
            None => NoOpAssertionHook::create_result(pass, message, results),
            Some(result) => results.push(result.or((pass, message).into())),
        }
    }
}
