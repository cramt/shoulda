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
fn something_really_important() {
    let expected = String::from("thingy");
    expected.should().be().equal(format!("{}ingy", "th"));
}

#[test]
fn is_math_real() {
    let expected = 4;
    expected.should().be().equal(2 + 2);
}
