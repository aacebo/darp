use proc_macro::TokenStream;
use quote::quote;

pub fn derive(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let ident = &input.ident;
    let len = data.fields.len();
    let indices: Vec<syn::Index> = (0..len).map(syn::Index::from).collect();
    let (impl_generics, type_generics, where_generics) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics ::darp::value::ToValue for #ident #type_generics #where_generics {
            fn to_value(&self) -> ::darp::value::Value {
                ::darp::value::Value::from_tuple(( #( self.#indices.to_value(), )* ))
            }
        }

        impl #impl_generics ::darp::value::Tuple for #ident #type_generics #where_generics {
            fn name(&self) -> &str {
                stringify!(#ident)
            }

            fn type_id(&self) -> ::std::any::TypeId {
                ::std::any::TypeId::of::<Self>()
            }

            fn len(&self) -> usize {
                #len
            }

            fn items(&self) -> ::darp::value::TupleIter<'_> {
                ::darp::value::TupleIter::new(
                    [#( &self.#indices as &dyn ::darp::value::ToValue, )*].into_iter()
                )
            }

            fn index(&self, i: usize) -> Option<&dyn ::darp::value::ToValue> {
                match i {
                    #( #indices => Some(&self.#indices as &dyn ::darp::value::ToValue), )*
                    _ => None,
                }
            }
        }
    }
    .into()
}
