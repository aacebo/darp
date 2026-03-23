use proc_macro::TokenStream;
use quote::quote;

pub fn derive(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let ident = &input.ident;
    let len = data.fields.len();
    let indices: Vec<syn::Index> = (0..len).map(syn::Index::from).collect();
    let (impl_generics, type_generics, where_generics) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics ::darp::reflect::ToValue for #ident #type_generics #where_generics {
            fn to_value(&self) -> ::darp::reflect::Value {
                ::darp::reflect::Value::from_tuple(( #( self.#indices.to_value(), )* ))
            }
        }

        impl #impl_generics ::darp::reflect::Tuple for #ident #type_generics #where_generics {
            fn name(&self) -> &str {
                stringify!(#ident)
            }

            fn type_id(&self) -> ::std::any::TypeId {
                ::std::any::TypeId::of::<Self>()
            }

            fn len(&self) -> usize {
                #len
            }

            fn items(&self) -> ::darp::reflect::TupleIter<'_> {
                ::darp::reflect::TupleIter::new(
                    [#( &self.#indices as &dyn ::darp::reflect::ToValue, )*].into_iter()
                )
            }

            fn index(&self, i: usize) -> Option<&dyn ::darp::reflect::ToValue> {
                match i {
                    #( #indices => Some(&self.#indices as &dyn ::darp::reflect::ToValue), )*
                    _ => None,
                }
            }
        }
    }
    .into()
}
