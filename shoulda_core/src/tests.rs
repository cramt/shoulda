use crate::Shoulda;
use std::borrow::Cow;

#[test]
fn vec_cow_i32() {
    vec![Cow::Borrowed(&1)].should().eq(vec![Cow::Owned(1)])
}

#[test]
fn vec_cow_str() {
    Cow::Borrowed("").should().eq(Cow::Borrowed(""));
    vec![Cow::Borrowed("")].should().eq(vec![Cow::Owned(String::new())]);
}
