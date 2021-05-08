# Shoulda
Shoulda is a BDD/TDD assertion library that adds to rusts pretty rudimentary assertions.

Shoulda is based on heavily [chai](https://www.chaijs.com/)'s `should` interface, as it makes you able to call `should` on almost any object and have a very readible chain of method calls.

---
```toml
shoulda = { git = "https://github.com/cramt/shoulda.git", branch = "main"}
```

## std examples
`should` can be called on most standard library types. However if there is an obvious type which is missing, please create an issue or pull request.
```rust
use shoulda::Shoulda;

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
```

for more std examples take a look at the tests within shoulda_core/src/tests.rs

## derive examples
Shoulda provides a derive macro which allows you to automatically implement Shoulda on all your core struct you wanna compare without requiring PartialEq.

It does require Debug as a proper error message cant be generated without it
```rust 
use shoulda::Shoulda;

#[derive(Debug, Shoulda)]
struct Person {
    name: String,
    has_toothbrush: bool,
}

#[test]
fn test() {
    Person {
        name: "Ronald".to_string(),
        has_toothbrush: false,
    }
    .should()
    .not()
    .be()
    .equal(Person {
        name: "Ronald".to_string(),
        has_toothbrush: true,
    });
}
```

for more derive examples take a look at the tests within tests/src/

## Float equality
For float equality this library checks wether or not the floats are within a specific threshold of eachother, that threshold i current defined as the envorimental variable `SHOULDA_FLOAT_DIFF_MODE` or if its not defined its defaulted to 0.0001.

This also happens when struct which Shoulda is applied to through the derive macro contains floating point numbers.