use proc_macro::TokenStream;
use quote::{ToTokens, quote};

use super::rules;
use super::schema_type::SchemaType;

pub fn derive(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_generics) = input.generics.split_for_impl();

    let field_stmts: Vec<proc_macro2::TokenStream> = data
        .fields
        .iter()
        .filter_map(|field| {
            let field_ident = field.ident.as_ref()?;
            let field_name = field_ident.to_string();
            let kind = SchemaType::from_type(&field.ty);
            let base = kind.to_token_stream();
            let attrs: Vec<_> = field
                .attrs
                .iter()
                .filter(|a| a.meta.path().is_ident("schema"))
                .collect();

            let mut rule_calls: Vec<proc_macro2::TokenStream> = Vec::new();

            for attr in &attrs {
                let punct: syn::punctuated::Punctuated<syn::Meta, syn::Token![,]> =
                    match attr.parse_args_with(syn::punctuated::Punctuated::parse_terminated) {
                        Ok(v) => v,
                        Err(e) => return Some(e.to_compile_error()),
                    };

                for meta in &punct {
                    rule_calls.push(
                        rules::parse_rule(meta, &kind).unwrap_or_else(|e| e.to_compile_error()),
                    );
                }
            }

            let schema_expr = if rule_calls.is_empty() {
                quote!(#base)
            } else {
                quote! {{
                    let schema = #base;
                    #(let schema = schema.#rule_calls;)*
                    schema
                }}
            };

            Some(quote! {
                schema = schema.field(#field_name, #schema_expr);
            })
        })
        .collect();

    quote! {
        impl #impl_generics ::darp::assert::ToSchema for #ident #type_generics #where_generics {
            fn to_schema(&self) -> ::darp::Schema {
                let mut schema = ::darp::assert::object();
                #(#field_stmts)*
                schema.into()
            }
        }

        impl #impl_generics ::darp::Validate for #ident #type_generics #where_generics {
            fn validate(&self) -> Result<::darp::Value, ::darp::assert::ValidError> {
                self.to_schema().validate(&::darp::ToValue::to_value(self).into())
            }
        }
    }
    .into()
}
