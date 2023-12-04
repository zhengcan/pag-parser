use super::{context::ParseContext, parser::Parser, ParseError};

/// 可解析对象
pub trait Parsable
where
    Self: Sized,
{
    /// 解析
    fn parse(parser: &mut impl Parser, ctx: impl ParseContext) -> Result<Self, ParseError>;
}

impl Parsable for f32 {
    #[inline(always)]
    fn parse(parser: &mut impl Parser, _ctx: impl ParseContext) -> Result<Self, ParseError> {
        parser.next_f32()
    }
}

impl Parsable for u8 {
    #[inline(always)]
    fn parse(parser: &mut impl Parser, _ctx: impl ParseContext) -> Result<Self, ParseError> {
        parser.next_u8()
    }
}

// impl Parsable for u32 {
//     #[inline(always)]
//     fn parse(parser: &mut impl Parser, _ctx: impl ParserContext) -> Result<Self, ParseError> {
//         parser.next_encoded_u32()
//     }
// }

// impl Parsable for u64 {
//     #[inline(always)]
//     fn parse(parser: &mut impl Parser, _ctx: impl ParserContext) -> Result<Self, ParseError> {
//         parser.next_encoded_u64()
//     }
// }

impl Parsable for bool {
    #[inline(always)]
    fn parse(parser: &mut impl Parser, _ctx: impl ParseContext) -> Result<Self, ParseError> {
        parser.next_bool()
    }
}

impl Parsable for String {
    #[inline(always)]
    fn parse(parser: &mut impl Parser, _ctx: impl ParseContext) -> Result<Self, ParseError> {
        parser.next_string()
    }
}

// #[deprecated]
// pub trait ContextualParsable: Parsable
// where
//     Self: Sized,
// {
//     fn parse(parser: &mut impl Parser) -> Result<Self, ParseError> {
//         Self::parse_with(parser, ())
//     }

//     /// 结合上下文一起解析
//     fn parse_with(parser: &mut impl Parser, _ctx: impl ParserContext) -> Result<Self, ParseError>;
// }
