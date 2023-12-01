use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ParsableEnum)]
pub fn derive_parsable_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    TokenStream::from(quote! {
        impl crate::format::Parsable for #name {
            #[inline(always)]
            fn parse_a(parser: &mut impl crate::parser::Parser) -> Result<Self, crate::parser::ParseError> {
                parser.next_enum()
            }
        }

        impl crate::format::AttributeValue for #name {
        }
    })
}
