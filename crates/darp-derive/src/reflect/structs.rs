use proc_macro::TokenStream;
use quote::quote;

pub fn derive(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let ident = &input.ident;
    let fields: Vec<_> = data.fields.iter().filter_map(|f| f.ident.clone()).collect();
    let len = fields.len();
    let (impl_generics, type_generics, where_generics) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics ::darp::reflect::ToValue for #ident #type_generics #where_generics {
            fn to_value(&self) -> ::darp::reflect::Value {
                let mut map = ::std::collections::BTreeMap::new();
                #(
                    map.insert(
                        ::darp::path::Ident::from(stringify!(#fields)),
                        self.#fields.to_value(),
                    );
                )*
                ::darp::reflect::Value::from_struct(map)
            }
        }

        impl #impl_generics ::darp::reflect::Struct for #ident #type_generics #where_generics {
            fn name(&self) -> &str {
                stringify!(#ident)
            }

            fn type_id(&self) -> ::std::any::TypeId {
                ::std::any::TypeId::of::<Self>()
            }

            fn len(&self) -> usize {
                #len
            }

            fn items(&self) -> ::darp::reflect::StructIter<'_> {
                static KEYS: ::std::sync::LazyLock<[::darp::path::Ident; #len]> =
                    ::std::sync::LazyLock::new(|| [
                        #(::darp::path::Ident::from(stringify!(#fields)),)*
                    ]);

                let values: [&dyn ::darp::reflect::ToValue; #len] = [
                    #(&self.#fields as &dyn ::darp::reflect::ToValue,)*
                ];

                ::darp::reflect::StructIter::new(
                    KEYS.iter().zip(values).map(|(k, v)| (k, v))
                )
            }

            fn field(&self, ident: ::darp::path::Ident) -> Option<&dyn ::darp::reflect::ToValue> {
                #(
                    if ident == stringify!(#fields) {
                        return Some(&self.#fields as &dyn ::darp::reflect::ToValue);
                    }
                )*

                None
            }
        }
    }
    .into()
}
