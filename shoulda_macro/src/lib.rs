use quote::quote;
use syn::__private::TokenStream2;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro]
pub fn expr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream2::from(input);
    let str_input = input.to_string();
    proc_macro::TokenStream::from(
        quote! {::shoulda::core::specifics::panic::Expression::new(#input, #str_input.to_string())},
    )
}

#[proc_macro_derive(Shoulda)]
pub fn shoulda(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let body = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(n) => n
                .named
                .iter()
                .map(|x| x.ident.as_ref().unwrap())
                .map(|x| {
                    quote! {
                        self.#x.should_eq::<Epsilon>(&other.#x)
                    }
                })
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" && ")
                .parse::<TokenStream2>()
                .unwrap(),
            Fields::Unnamed(u) => u
                .unnamed
                .iter()
                .enumerate()
                .map(|(x, _)| x.to_string().parse::<TokenStream2>().unwrap())
                .map(|x| {
                    quote! {
                        self.#x.should_eq::<Epsilon>(&other.#x)
                    }
                })
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" && ")
                .parse::<TokenStream2>()
                .unwrap(),
            Fields::Unit => quote! {
                true
            },
        },
        Data::Enum(e) => {
            let matches = e.variants.iter().map(|x| {
                let variant = &x.ident;
                match &x.fields {
                    Fields::Named(_) => {
                    let size = x.fields.iter().enumerate().map(|x| (x.1.ident.as_ref().unwrap().to_string(), format!("__{}",  x.0))).collect::<Vec<(String, String)>>();
                    let a_var_args = format!("{{{}}}", size.iter().map(|(a,b)|format!("{}:{}a", a,b)).collect::<Vec<String>>().join(","));
                    let b_var_args = format!("{{{}}}", size.iter().map(|(a,b)|format!("{}:{}b", a,b)).collect::<Vec<String>>().join(","));
                    let eval: String = size.iter().map(|x|format!("{0}a.should_eq::<Epsilon>({0}b)", x.1)).collect::<Vec<String>>().join(" && ");
                    format!("({name}::{variant}{a_var_args}, {name}::{variant}{b_var_args}) => {eval}, ",
                            name = name, variant = variant, a_var_args = a_var_args, b_var_args = b_var_args, eval = eval)
                        .parse::<TokenStream2>()
                        .unwrap()
                    }
                    Fields::Unnamed(_) => {
                    let size = x.fields.iter().enumerate().map(|x| format!("__{}", x.0)).collect::<Vec<String>>();
                    let a_var_args = format!("({})", size.iter().map(|x|format!("{}a", x)).collect::<Vec<String>>().join(","));
                    let b_var_args = format!("({})", size.iter().map(|x|format!("{}b", x)).collect::<Vec<String>>().join(","));
                    let eval: String = size.iter().map(|x|format!("{0}a.should_eq::<Epsilon>({0}b)", x)).collect::<Vec<String>>().join(" && ");
                    format!("({name}::{variant}{a_var_args}, {name}::{variant}{b_var_args}) => {eval}, ",
                            name = name, variant = variant, a_var_args = a_var_args, b_var_args = b_var_args, eval = eval)
                        .parse::<TokenStream2>()
                        .unwrap()
                    }
                    Fields::Unit => quote! {
                        (#name::#variant, #name::#variant) => true,
                    }
                }
            }).fold(String::new(), |acc, x| {
                acc + x.to_string().as_str()
            }).parse::<TokenStream2>().unwrap();
            quote! {
                match (self, other) {
                    #matches
                    _ => false
                }
            }
        }
        Data::Union(_) => panic!("assertable union types not supported"),
    };

    let generics = input.generics;

    let expanded = quote! {
        impl#generics ::shoulda::core::shoulda_equal::ShouldaEqual for #name#generics {
            fn should_eq<Epsilon: ::shoulda::core::epsilon_provider::EpsilonProvider>(&self, other: &Self) -> bool {
                #body
            }
        }
    };
    //panic!("{}", expanded.to_string());
    proc_macro::TokenStream::from(expanded)
}
