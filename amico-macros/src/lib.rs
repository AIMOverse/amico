use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(WithParams, attributes(params))]
pub fn derive_with_params(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Find the field marked with #[params]
    let params_field = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.iter().find(|field| {
                field
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("params"))
            }),
            _ => panic!("WithParams can only be derived for structs with named fields"),
        },
        _ => panic!("WithParams can only be derived for structs"),
    };

    let params_field = params_field.expect("No field marked with #[params] found");
    let params_field_name = &params_field.ident;

    let expanded = quote! {
        impl #name {
            pub fn param(&self, key: &str) -> Option<&toml::Value> {
                let Some(params) = self.#params_field_name.as_ref() else {
                    return None;
                };

                params.get(key)
            }
        }
    };

    TokenStream::from(expanded)
}
