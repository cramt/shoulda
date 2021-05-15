use crate::float_diff_provider::ConstFloatDiffProvider;
use crate::Shoulda;
use std::borrow::Cow;

#[test]
fn vec_cow_i32() {
    vec![Cow::Borrowed(&1)].should().eq(vec![Cow::Owned(1)]);
}

#[test]
fn vec_cow_str() {
    Cow::Borrowed("").should().eq(Cow::Borrowed(""));
    vec![Cow::Borrowed("")]
        .should()
        .eq(vec![Cow::Owned(String::new())]);
}

#[test]
fn contains() {
    vec![1, 2, 3].should().contains(|x| x.eq(&2));
}

#[test]
fn float_diff_changes() {
    //u64 rep of 0.1 f64
    const N: u64 = 4591870180066957722;
    1f64.should()
        .float_diff::<ConstFloatDiffProvider<N>>()
        .eq(1.09f64);
}

#[test]
fn should_panic_on_fail() {
    let previous_hook = std::panic::take_hook();
    // Override the default hook to avoid logging panic location info.
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(|| {
        let a = 1;
        a.should().eq(0);
    });
    std::panic::set_hook(previous_hook);
    result.is_err().should().be_true();
}

#[test]
fn or_operation() {
    1i32.should().eq(2).or().eq(1);
}

#[test]
fn operation_order_dependent() {
    let previous_hook = std::panic::take_hook();
    // Override the default hook to avoid logging panic location info.
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(|| {
        let thingy: Option<i32> = None;
        thingy.should().be().some();
        thingy.unwrap().should().be().eq(2);
    });
    std::panic::set_hook(previous_hook);
    result.is_err().should().be_true();
    result
        .err()
        .unwrap()
        .downcast_ref::<String>()
        .unwrap()
        .should()
        .eq("None is None, Some expected".to_string());
}
