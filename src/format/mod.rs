mod attrs;
mod base;
mod image;
mod layer;
mod primitive;
mod shape;
mod tag;
mod text;
mod video;

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

pub trait ParserContext: Copy {
    fn as_bool(&self) -> bool;
}

impl ParserContext for () {
    fn as_bool(&self) -> bool {
        false
    }
}

impl ParserContext for bool {
    fn as_bool(&self) -> bool {
        *self
    }
}

/// 从流中解析
pub trait StreamParser
where
    Self: Sized,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self>;

    fn parse_and<F>(input: &[u8], f: F) -> IResult<&[u8], Self>
    where
        F: Fn(&mut Self),
    {
        Self::parse(input)
    }

    fn parse_with(input: &[u8], ctx: impl ParserContext) -> IResult<&[u8], Self> {
        Self::parse(input)
    }

    fn parse_block(input: &[u8]) -> Result<Self, nom::Err<nom::error::Error<&[u8]>>> {
        Self::parse_block_with(input, ())
    }

    fn parse_block_with(
        input: &[u8],
        ctx: impl ParserContext,
    ) -> Result<Self, nom::Err<nom::error::Error<&[u8]>>> {
        Self::parse_with(input, ctx).map(|(remain, v)| {
            assert_eq!(remain.len(), 0);
            v
        })
    }

    fn try_from_bool(value: bool) -> Option<Self> {
        None
    }

    fn try_from_key_frames(value: Vec<String>) -> Option<Self> {
        None
    }
}

/// Pag 文件格式
#[derive(Debug)]
pub struct Pag {
    pub header: FileHeader,
    pub tag_block: TagBlock,
}

impl StreamParser for Pag {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (header, tag_block)) = tuple((FileHeader::parse, TagBlock::parse))(input)?;
        Ok((input, Self { header, tag_block }))
    }
}

/// Pag 文件头
#[derive(Debug)]
pub struct FileHeader {
    // pub magic: [u8; 3],
    pub version: u8,
    pub length: u32,
    pub compress_method: i8,
}

impl StreamParser for FileHeader {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (_, version, length, compress_method)) =
            tuple((tag("PAG"), le_u8, le_u32, le_i8))(input)?;
        let header = Self {
            // magic: [b'P', b'A', b'G'],
            version,
            length,
            compress_method,
        };
        Ok((input, header))
    }
}
