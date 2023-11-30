use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Parsable)]
pub fn derive_parsable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    TokenStream::from(quote! {
        impl crate::format::StreamParser for #name {
            #[inline(always)]
            fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
                crate::format::parse_enum(input)
            }
        }
    })
}
