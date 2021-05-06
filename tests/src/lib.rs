#[cfg(test)]
mod tests {
    mod empty_enum {
        use assertable::Assertable;

        #[derive(Debug, Assertable)]
        enum EmptyEnum {
            A,
            B,
            C,
        }

        #[test]
        fn same_variant_matches() {
            EmptyEnum::A.assert_eq(&EmptyEnum::A);
            EmptyEnum::B.assert_eq(&EmptyEnum::B);
            EmptyEnum::C.assert_eq(&EmptyEnum::C);
        }

        #[test]
        fn different_variant_doesnt_match() {
            EmptyEnum::A.assert_ne(&EmptyEnum::B);
            EmptyEnum::A.assert_ne(&EmptyEnum::C);
            EmptyEnum::B.assert_ne(&EmptyEnum::A);
            EmptyEnum::B.assert_ne(&EmptyEnum::C);
            EmptyEnum::C.assert_ne(&EmptyEnum::A);
            EmptyEnum::C.assert_ne(&EmptyEnum::B);
        }
    }

    mod full_enum {
        use assertable::Assertable;

        #[derive(Debug, Assertable)]
        enum FullEnum {
            A(i32),
            B(String),
            C(Result<usize, ()>),
        }

        #[test]
        fn variant_and_value_match() {
            FullEnum::A(42).assert_eq(&FullEnum::A(42));
            FullEnum::B("a".to_string()).assert_eq(&FullEnum::B("a".to_string()));
            FullEnum::C(Err(())).assert_eq(&FullEnum::C(Err(())));
        }

        #[test]
        fn variant_matches_and_value_doesnt_match() {
            FullEnum::A(42).assert_ne(&FullEnum::A(41));
            FullEnum::B("a".to_string()).assert_ne(&FullEnum::B("b".to_string()));
            FullEnum::C(Ok(1)).assert_ne(&FullEnum::C(Ok(2)));
        }

        #[test]
        fn variants_doesnt_doesnt_match() {
            FullEnum::A(42).assert_ne(&FullEnum::B("a".to_string()));
            FullEnum::A(42).assert_ne(&FullEnum::C(Err(())));
            FullEnum::B("a".to_string()).assert_ne(&FullEnum::A(42));
            FullEnum::B("a".to_string()).assert_ne(&FullEnum::C(Err(())));
            FullEnum::C(Err(())).assert_ne(&FullEnum::B("a".to_string()));
            FullEnum::C(Err(())).assert_ne(&FullEnum::A(42));
        }
    }

    mod named_structs {
        use assertable::Assertable;

        #[derive(Debug, Assertable)]
        struct Struct {
            something: String,
            x: usize,
            things: Vec<usize>,
        }

        #[test]
        fn struct_matches() {
            Struct {
                something: "a".to_string(),
                x: 2,
                things: vec![7],
            }
            .assert_eq(&Struct {
                something: "a".to_string(),
                x: 2,
                things: vec![7],
            })
        }
    }

    mod lifetimes {
        use assertable::Assertable;
        use std::borrow::Cow;

        #[derive(Debug, Assertable)]
        struct LifeTimeStruct<'a>(Cow<'a, str>);

        #[test]
        fn lifetime_struct_matches() {
            LifeTimeStruct("".into()).assert_eq(&LifeTimeStruct("".into()))
        }
    }
}
