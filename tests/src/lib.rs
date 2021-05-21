#[cfg(test)]
mod tests {
    mod empty_enum {
        use shoulda::Shoulda;

        #[derive(Debug, Shoulda)]
        enum EmptyEnum {
            A,
            B,
            C,
        }

        #[test]
        fn same_variant_matches() {
            EmptyEnum::A.should().eq(EmptyEnum::A);
            EmptyEnum::B.should().eq(EmptyEnum::B);
            EmptyEnum::C.should().eq(EmptyEnum::C);
        }

        #[test]
        fn different_variant_doesnt_match() {
            EmptyEnum::A.should().not().eq(EmptyEnum::B);
            EmptyEnum::A.should().not().eq(EmptyEnum::C);
            EmptyEnum::B.should().not().eq(EmptyEnum::A);
            EmptyEnum::B.should().not().eq(EmptyEnum::C);
            EmptyEnum::C.should().not().eq(EmptyEnum::A);
            EmptyEnum::C.should().not().eq(EmptyEnum::B);
        }
    }

    mod full_enum {
        use shoulda::Shoulda;

        #[derive(Debug, Shoulda)]
        enum FullEnum {
            A(i32),
            B(String),
            C(Result<usize, ()>),
        }

        #[test]
        fn variant_and_value_match() {
            FullEnum::A(42).should().eq(FullEnum::A(42));
            FullEnum::B("a".to_string())
                .should()
                .eq(FullEnum::B("a".to_string()));
            FullEnum::C(Err(())).should().eq(FullEnum::C(Err(())));
        }

        #[test]
        fn variant_matches_and_value_doesnt_match() {
            FullEnum::A(42).should().not().eq(FullEnum::A(41));
            FullEnum::B("a".to_string())
                .should()
                .not()
                .eq(FullEnum::B("b".to_string()));
            FullEnum::C(Ok(1)).should().not().eq(FullEnum::C(Ok(2)));
        }

        #[test]
        fn variants_doesnt_doesnt_match() {
            FullEnum::A(42)
                .should()
                .not()
                .eq(FullEnum::B("a".to_string()));
            FullEnum::A(42).should().not().eq(FullEnum::C(Err(())));
            FullEnum::B("a".to_string())
                .should()
                .not()
                .eq(FullEnum::A(42));
            FullEnum::B("a".to_string())
                .should()
                .not()
                .eq(FullEnum::C(Err(())));
            FullEnum::C(Err(()))
                .should()
                .not()
                .eq(FullEnum::B("a".to_string()));
            FullEnum::C(Err(())).should().not().eq(FullEnum::A(42));
        }
    }

    mod named_structs {
        use shoulda::Shoulda;

        #[derive(Debug, Shoulda)]
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
            .should()
            .eq(Struct {
                something: "a".to_string(),
                x: 2,
                things: vec![7],
            });
        }
    }

    mod lifetimes {
        use shoulda::Shoulda;
        use std::borrow::Cow;

        #[derive(Debug, Shoulda)]
        struct LifeTimeStruct<'a>(Cow<'a, str>);

        #[test]
        fn lifetime_struct_matches() {
            LifeTimeStruct("".into())
                .should()
                .eq(LifeTimeStruct("".into()));
        }
    }

    mod highly_nested_generic {
        use shoulda::Shoulda;

        #[derive(Debug, Shoulda)]
        struct GenericHell(Vec<Option<Vec<Result<u32, ()>>>>);

        #[test]
        fn generic_hell() {
            GenericHell(vec![Some(vec![Ok(2)])])
                .should()
                .eq(GenericHell(vec![Some(vec![Ok(2)])]));
        }
    }

    mod optional_cow {
        use shoulda::Shoulda;
        use std::borrow::Cow;

        #[derive(Debug, Shoulda)]
        struct OptionalCow<'a>(Option<Cow<'a, i32>>);

        #[test]
        fn optional_cow() {
            OptionalCow(Some(Cow::Owned(2)))
                .should()
                .eq(OptionalCow(Some(Cow::Borrowed(&2))));
        }
    }

    mod vecs {
        use shoulda::Shoulda;

        #[derive(Debug, Shoulda)]
        struct Vecs<'a>(Vec<&'a u64>);

        #[test]
        fn vecs() {
            Vecs(vec![&1, &2, &3]).should().eq(Vecs(vec![&1, &2, &3]));
        }
    }

    mod vec_of_cows {
        use shoulda::Shoulda;
        use std::borrow::Cow;

        #[derive(Debug, Shoulda)]
        struct Vecs<'a>(Vec<Cow<'a, u64>>);

        #[test]
        fn vec_of_cows() {
            Vecs(vec![Cow::Borrowed(&1), Cow::Owned(2), Cow::Borrowed(&3)])
                .should()
                .eq(Vecs(vec![Cow::Owned(1), Cow::Borrowed(&2), Cow::Owned(3)]));
        }
    }

    mod vec_of_cow_strs {
        use shoulda::Shoulda;
        use std::borrow::Cow;

        #[derive(Debug, Shoulda)]
        struct Vecs<'a>(Vec<Cow<'a, str>>);

        #[test]
        fn vec_of_cow_strs() {
            Vecs(vec![
                Cow::Borrowed("a"),
                Cow::Owned("b".to_string()),
                Cow::Borrowed("c"),
            ])
            .should()
            .eq(Vecs(vec![
                Cow::Owned("a".to_string()),
                Cow::Borrowed("b"),
                Cow::Owned("c".to_string()),
            ]));
        }
    }

    mod expect {
        use shoulda::expect;

        #[test]
        fn expect_works() {
            expect!(2).to().be().eq(2);
        }
    }

    mod panic {
        use shoulda::expr;
        use shoulda::Shoulda;

        #[test]
        fn expr() {
            ().should().panic_with(
                |_: &_| {
                    vec![1].should().panic(expr!(|a: &Vec<i32>| {
                        let _ = &a[0];
                    }));
                },
                "|a: &Vec<i32>| { let _ = &a[0]; } didnt panic",
            );
        }
    }
}
