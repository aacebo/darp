mod assert;
mod reflect;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Value)]
pub fn derive_value(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(_) => reflect::structs::derive(&input, data),
            syn::Fields::Unnamed(_) => reflect::tuples::derive(&input, data),
            syn::Fields::Unit => {
                let ident = &input.ident;
                let (ig, tg, wg) = input.generics.split_for_impl();
                quote! {
                    impl #ig ::darp::reflect::ToValue for #ident #tg #wg {
                        fn to_value(&self) -> ::darp::reflect::Value {
                            ::darp::reflect::Value::Null
                        }
                    }
                }
                .into()
            }
        },
        syn::Data::Enum(data) => reflect::enums::derive(&input, data),
        _ => syn::Error::new_spanned(&input, "Value cannot be derived for unions")
            .to_compile_error()
            .into(),
    }
}

#[proc_macro_derive(Validate, attributes(schema))]
pub fn derive_validate(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(data) => assert::structs::derive(&input, data),
        _ => syn::Error::new_spanned(&input, "Validate can only be derived for structs")
            .to_compile_error()
            .into(),
    }
}
