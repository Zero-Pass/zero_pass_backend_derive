use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

#[proc_macro_derive(Method)]
pub fn method_derive(input: TokenStream) -> TokenStream {
    let enum_input = parse_macro_input!(input as ItemEnum);
    let enum_name = &enum_input.ident;

    let mut to_method_arms = quote! {};
    let mut try_from_arms = quote! {};
    let mut method_names = quote!{};

    for variant in &enum_input.variants {
        let variant_name = &variant.ident;
        let variant_name_str = &variant.ident.to_string();

        method_names = quote! {
            #method_names
            #variant_name_str,
        };

        let variant_name_str = variant_name_str.to_lowercase();
        let variant_name_str = variant_name_str.as_str();

        to_method_arms = quote! {
            #to_method_arms
            #enum_name::#variant_name => Box::new(#variant_name),
        };

        try_from_arms = quote! {
            #try_from_arms
            #variant_name_str => Ok(#enum_name::#variant_name),
        };
    }

    let expanded = quote! {

        impl TryFrom<String> for #enum_name {
            type Error = Error;

            fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
                match value.to_lowercase().as_str() {
                    #try_from_arms
                    _ => Err(Error::InvalidMethodError(value)),
                }
            }
        }


        impl #enum_name {
            pub fn to_method(&self) -> Box<dyn Method> {
                match self {
                    #to_method_arms
                }
            }

            pub fn get_methods<'a>() -> Vec<&'a str> {
                let methods = vec![#method_names];
                methods

            }
        }

    };

    expanded.into()
}
