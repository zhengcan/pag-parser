mod attrs;
mod base;
mod image;
mod layer;
mod primitive;
mod shape;
mod tag;
mod text;
mod video;

use std::{fmt::Debug, rc::Rc};

use nom::{bytes::complete::tag, number::complete::*, sequence::tuple, IResult};

pub use attrs::*;
pub use base::*;
pub use image::*;
pub use layer::*;
pub use primitive::*;
pub use shape::*;
pub use tag::*;
pub use text::*;
pub use video::*;

use crate::parser::{ParseError, Parser};

pub trait ParserContext: Clone {
    fn with_tag_code(&self, tag_code: TagCode) -> impl ParserContext;
    fn with_layer_type(&self, layer_type: LayerType) -> impl ParserContext;

    fn as_bool(&self) -> bool;
    fn parent_code(&self) -> Option<TagCode>;
    fn layer_type(&self) -> Option<LayerType>;
}

#[derive(Debug, Clone)]
struct DefaultParserContext {
    tag_code: Option<TagCode>,
    layer_type: Option<LayerType>,
}

impl ParserContext for DefaultParserContext {
    fn with_tag_code(&self, tag_code: TagCode) -> impl ParserContext {
        Self {
            tag_code: Some(tag_code),
            ..self.clone()
        }
    }

    fn with_layer_type(&self, layer_type: LayerType) -> impl ParserContext {
        Self {
            layer_type: Some(layer_type),
            ..self.clone()
        }
    }

    fn as_bool(&self) -> bool {
        false
    }

    fn parent_code(&self) -> Option<TagCode> {
        None
    }

    fn layer_type(&self) -> Option<LayerType> {
        self.layer_type
    }
}

impl ParserContext for () {
    fn with_tag_code(&self, tag_code: TagCode) -> impl ParserContext {
        DefaultParserContext {
            tag_code: Some(tag_code),
            layer_type: None,
        }
    }

    fn with_layer_type(&self, layer_type: LayerType) -> impl ParserContext {
        DefaultParserContext {
            tag_code: None,
            layer_type: Some(layer_type),
        }
    }

    fn as_bool(&self) -> bool {
        false
    }

    fn parent_code(&self) -> Option<TagCode> {
        None
    }

    fn layer_type(&self) -> Option<LayerType> {
        None
    }
}

impl ParserContext for bool {
    fn with_tag_code(&self, tag_code: TagCode) -> impl ParserContext {
        DefaultParserContext {
            tag_code: Some(tag_code),
            layer_type: None,
        }
    }

    fn with_layer_type(&self, layer_type: LayerType) -> impl ParserContext {
        DefaultParserContext {
            tag_code: None,
            layer_type: Some(layer_type),
        }
    }

    fn as_bool(&self) -> bool {
        *self
    }

    fn parent_code(&self) -> Option<TagCode> {
        None
    }

    fn layer_type(&self) -> Option<LayerType> {
        None
    }
}

pub trait Parsable
where
    Self: Sized,
{
    fn parse(parser: &mut impl Parser) -> Result<Self, ParseError>;
}

impl Parsable for f32 {
    #[inline(always)]
    fn parse(parser: &mut impl Parser) -> Result<Self, ParseError> {
        parser.next_f32()
    }
}

impl Parsable for u8 {
    #[inline(always)]
    fn parse(parser: &mut impl Parser) -> Result<Self, ParseError> {
        parser.next_u8()
    }
}

impl Parsable for u32 {
    #[inline(always)]
    fn parse(parser: &mut impl Parser) -> Result<Self, ParseError> {
        parser.next_encoded_u32()
    }
}

impl Parsable for u64 {
    #[inline(always)]
    fn parse(parser: &mut impl Parser) -> Result<Self, ParseError> {
        parser.next_encoded_u64()
    }
}

impl Parsable for bool {
    #[inline(always)]
    fn parse(parser: &mut impl Parser) -> Result<Self, ParseError> {
        parser.next_bool()
    }
}

impl Parsable for String {
    #[inline(always)]
    fn parse(parser: &mut impl Parser) -> Result<Self, ParseError> {
        parser.next_string()
    }
}

pub trait ContextualParsable
where
    Self: Sized,
{
    fn parse_b(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError>;
}

/// 从流中解析
// #[deprecated]
// pub trait StreamParser
// where
//     Self: Sized,
// {
//     fn parse(input: &[u8]) -> IResult<&[u8], Self>;

//     fn parsend<F>(input: &[u8], f: F) -> IResult<&[u8], Self>
//     where
//         F: Fn(&mut Self),
//     {
//         Self::parse(input)
//     }

//     fn parse_with(input: &[u8], ctx: impl ParserContext) -> IResult<&[u8], Self> {
//         Self::parse(input)
//     }

//     fn parse_block(input: &[u8]) -> Result<Self, nom::Err<nom::error::Error<&[u8]>>> {
//         Self::parse_block_with(input, ())
//     }

//     fn parse_block_with(
//         input: &[u8],
//         ctx: impl ParserContext,
//     ) -> Result<Self, nom::Err<nom::error::Error<&[u8]>>> {
//         Self::parse_with(input, ctx).map(|(remain, v)| {
//             assert_eq!(remain.len(), 0);
//             v
//         })
//     }

//     fn try_from_bool(value: bool) -> Option<Self> {
//         None
//     }

//     fn try_from_key_frames(value: Vec<String>) -> Option<Self> {
//         None
//     }
// }

/// Pag 文件格式
#[derive(Debug)]
pub struct Pag {
    pub header: FileHeader,
    pub tag_block: TagBlock,
}

/// Pag 文件头
#[derive(Debug)]
pub struct FileHeader {
    // pub magic: [u8; 3],
    pub version: u8,
    pub length: u32,
    pub compress_method: i8,
}

impl Parsable for FileHeader {
    fn parse(parser: &mut impl Parser) -> Result<Self, ParseError> {
        let _ = parser.next_term("PAG")?;
        let version = parser.next_u8()?;
        let length = parser.next_u32()?;
        let compress_method = parser.next_i8()?;
        Ok(Self {
            version,
            length,
            compress_method,
        })
    }
}

// impl StreamParser for FileHeader {
//     fn parse(input: &[u8]) -> IResult<&[u8], Self> {
//         let (input, (_, version, length, compress_method)) =
//             tuple((tag("PAG"), le_u8, le_u32, le_i8))(input)?;
//         let header = Self {
//             // magic: [b'P', b'A', b'G'],
//             version,
//             length,
//             compress_method,
//         };
//         Ok((input, header))
//     }
// }
