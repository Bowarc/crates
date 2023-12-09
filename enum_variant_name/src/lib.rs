use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

#[proc_macro_derive(VariantName)]
pub fn variant_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let data = &input.data;

    let match_arms = match data {
        Data::Enum(enum_data) => {
            let match_arms = enum_data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let variant_string = variant_ident.to_string();
                let fields_in_variant = quote! { {..} };
                quote! {
                    Self::#variant_ident #fields_in_variant => #variant_string,
                }
            });

            quote! {
                match self {
                    #(#match_arms)*
                }
            }
        }
        _ => {
            return Error::new_spanned(&input, "VariantName can only be implemented for enums")
                .to_compile_error()
                .into();
        }
    };

    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn variant_name(&self) -> &'static str {
                #match_arms
            }
        }
    };

    TokenStream::from(expanded)
}