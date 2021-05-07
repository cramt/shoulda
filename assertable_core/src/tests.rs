use std::borrow::Cow;
use crate::Assertable;

#[test]
fn vec_cow_i32(){
    vec![Cow::Borrowed(&1)].assert_eq(&vec![Cow::Owned(1)])
}

#[test]
fn vec_cow_str(){
    Cow::Borrowed("").assert_eq(&Cow::Owned(String::new()));
    vec![Cow::Borrowed("")].assert_eq(&vec![Cow::Owned(String::new())]);
}
