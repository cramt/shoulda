use crate::Shoulda;
use std::borrow::Cow;
use crate::float_diff_provider::ConstFloatDiffProvider;

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
fn float_diff_changes(){
    //u64 rep of 0.1 f64
    const N: u64 = 4591870180066957722;
    1f64.should().float_diff::<ConstFloatDiffProvider<N>>().eq(1.09f64);
}
