mod schema;
mod value;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Value)]
pub fn derive_value(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(_) => value::structs::derive(&input, data),
            syn::Fields::Unnamed(_) => value::tuples::derive(&input, data),
            syn::Fields::Unit => {
                let ident = &input.ident;
                let (ig, tg, wg) = input.generics.split_for_impl();
                quote! {
                    impl #ig ::darp::value::ToValue for #ident #tg #wg {
                        fn to_value(&self) -> ::darp::value::Value {
                            ::darp::value::Value::Null
                        }
                    }
                }
                .into()
            }
        },
        syn::Data::Enum(data) => value::enums::derive(&input, data),
        _ => syn::Error::new_spanned(&input, "Value cannot be derived for unions")
            .to_compile_error()
            .into(),
    }
}

#[proc_macro_derive(Validate, attributes(schema))]
pub fn derive_validate(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => schema::structs::derive(&input, data),
        _ => syn::Error::new_spanned(&input, "Validate can only be derived for structs")
            .to_compile_error()
            .into(),
    }
}
