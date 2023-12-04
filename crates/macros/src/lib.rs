use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ParsableEnum)]
pub fn derive_parsable_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    TokenStream::from(quote! {
        impl crate::parse::Parsable for #name {
            #[inline(always)]
            fn parse(parser: &mut impl crate::parse::Parser, _ctx: impl crate::parse::ParserContext) -> Result<Self, crate::parse::ParseError> {
                parser.next_enum()
            }
        }

        impl crate::parse::AttributeValue for #name {
        }
    })
}
