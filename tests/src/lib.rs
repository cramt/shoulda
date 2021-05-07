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

    mod highly_nested_generic {
        use assertable::Assertable;

        #[derive(Debug, Assertable)]
        struct GenericHell(Vec<Option<Vec<Result<u32, ()>>>>);

        #[test]
        fn generic_hell() {
            GenericHell(vec![Some(vec![Ok(2)])])
                .assert_eq(&GenericHell(vec![Some(vec![Ok(2)])]))
        }
    }

    mod optional_cow {
        use assertable::Assertable;
        use std::borrow::Cow;

        #[derive(Debug, Assertable)]
        struct OptionalCow<'a>(Option<Cow<'a, i32>>);

        #[test]
        fn optional_cow() {
            OptionalCow(Some(Cow::Owned(2)))
                .assert_eq(&OptionalCow(Some(Cow::Borrowed(&2))))
        }
    }

    mod vecs {
        use assertable::Assertable;

        #[derive(Debug, Assertable)]
        struct Vecs<'a>(Vec<&'a u64>);

        #[test]
        fn vecs() {
            Vecs(vec![&1, &2, &3]).assert_eq(&Vecs(vec![&1, &2, &3]))
        }
    }

    mod vec_of_cows {
        use assertable::Assertable;
        use std::borrow::Cow;

        #[derive(Debug, Assertable)]
        struct Vecs<'a>(Vec<Cow<'a, u64>>);

        #[test]
        fn vec_of_cows() {
            Vecs(vec![Cow::Borrowed(&1), Cow::Owned(2), Cow::Borrowed(&3)])
                .assert_eq(&Vecs(vec![Cow::Owned(1), Cow::Borrowed(&2), Cow::Owned(3)]))
        }
    }
}
