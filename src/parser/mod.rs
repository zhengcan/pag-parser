use std::cmp::min;
use std::fmt::Debug;

use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_i32, le_i64, le_u16, le_u32, le_u64, le_u8},
    Needed,
};
use thiserror::Error;

use crate::format::{
    parse_encode_i32, parse_encode_i64, parse_encode_u32, parse_encode_u64, parse_enum,
    parse_string, AttributeBlock, Bits, ByteData, Color, ContextualParsable, FileHeader, Parsable,
    ParserContext, Point, StreamParser, Tag, TagBlock, TagCode, Time,
};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("")]
    UnsupportPagVersion(u8),
    #[error("")]
    Incomplete(nom::Needed),
    #[error("")]
    BadFrame(nom::error::ErrorKind),
    #[error("")]
    IoError(#[from] std::io::Error),
    #[error("")]
    Eof,
}

impl<'a> From<nom::Err<nom::error::Error<&'a [u8]>>> for ParseError {
    fn from(error: nom::Err<nom::error::Error<&'a [u8]>>) -> Self {
        match error {
            nom::Err::Incomplete(needed) => ParseError::Incomplete(needed),
            nom::Err::Error(e) => ParseError::BadFrame(e.code),
            nom::Err::Failure(e) => ParseError::BadFrame(e.code),
        }
    }
}

pub trait Parser {
    fn buffer(&self) -> &[u8];

    #[inline(always)]
    fn remain(&self) -> usize {
        self.buffer().len()
    }

    #[inline(always)]
    fn peek(&self, max_length: usize) -> &[u8] {
        let buffer = self.buffer();
        &buffer[0..min(max_length, buffer.len())]
    }

    fn advance(&mut self, count: usize);

    fn new_slice(&mut self, length: usize) -> Result<impl Parser, ParseError>;

    #[inline(always)]
    fn new_attribute_block<'a>(&'a self) -> AttributeBlock<'a> {
        AttributeBlock::new(self.buffer())
    }

    #[inline(always)]
    fn new_bits<'a>(&'a mut self) -> Bits<'a> {
        Bits::new(self.buffer())
    }

    fn next_u8(&mut self) -> Result<u8, ParseError>;
    fn next_u16(&mut self) -> Result<u16, ParseError>;
    fn next_u32(&mut self) -> Result<u32, ParseError>;
    fn next_i32(&mut self) -> Result<i32, ParseError>;
    fn next_u64(&mut self) -> Result<u64, ParseError>;
    fn next_i64(&mut self) -> Result<i64, ParseError>;
    fn next_f32(&mut self) -> Result<f32, ParseError>;
    fn next_bool(&mut self) -> Result<bool, ParseError>;

    fn next_encoded_u32(&mut self) -> Result<u32, ParseError>;
    fn next_encoded_i32(&mut self) -> Result<i32, ParseError>;
    fn next_encoded_u64(&mut self) -> Result<u64, ParseError>;
    fn next_encoded_i64(&mut self) -> Result<i64, ParseError>;

    fn next_enum<T>(&mut self) -> Result<T, ParseError>
    where
        T: From<u8> + Debug;
    fn next_string(&mut self) -> Result<String, ParseError>;

    #[inline(always)]
    fn next_id(&mut self) -> Result<u32, ParseError> {
        self.next_encoded_u32()
    }
    #[inline(always)]
    fn next_time(&mut self) -> Result<Time, ParseError> {
        self.next_u64()
    }

    #[inline(always)]
    fn next_color(&mut self) -> Result<Color, ParseError> {
        let red = self.next_u8()?;
        let green = self.next_u8()?;
        let blue = self.next_u8()?;
        Ok(Color { red, green, blue })
    }

    #[inline(always)]
    fn next_point(&mut self) -> Result<Point, ParseError> {
        let x = self.next_f32()?;
        let y = self.next_f32()?;
        Ok(Point { x, y })
    }

    fn next_bytes(&mut self, count: usize) -> Result<&[u8], ParseError>;

    fn next_tag_block(&mut self, ctx: impl ParserContext) -> Result<TagBlock, ParseError>;

    // fn parse<T>(&mut self) -> Result<T, ParseError>
    // where
    //     T: Parsable,
    // {
    //     T::parse_a(self)
    // }
}

#[derive(Debug)]
pub struct SliceParser<'a> {
    input: &'a [u8],
}

impl<'a> SliceParser<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { input }
    }
}

impl<'a> Parser for SliceParser<'a> {
    fn buffer(&self) -> &[u8] {
        self.input
    }

    fn advance(&mut self, count: usize) {
        self.input = &self.input[count..];
    }

    fn new_slice(&mut self, length: usize) -> Result<impl Parser, ParseError> {
        let (input, slice) = take(length)(self.input)?;
        self.input = input;
        Ok(SliceParser { input: slice })
    }

    // fn close_bits<'b>(&'b mut self, bits: Bits<'b>)
    // where
    //     'a: 'b,
    // {
    //     let input = bits.finish();
    //     self.input = input;
    // }
    // fn reset<'b>(&'b mut self, input: &'b [u8]) {
    //     self.input = input;
    // }

    fn next_u8(&mut self) -> Result<u8, ParseError> {
        let (input, value) = le_u8(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_u16(&mut self) -> Result<u16, ParseError> {
        let (input, value) = le_u16(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_u32(&mut self) -> Result<u32, ParseError> {
        let (input, value) = le_u32(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_i32(&mut self) -> Result<i32, ParseError> {
        let (input, value) = le_i32(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_u64(&mut self) -> Result<u64, ParseError> {
        let (input, value) = le_u64(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_i64(&mut self) -> Result<i64, ParseError> {
        let (input, value) = le_i64(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_f32(&mut self) -> Result<f32, ParseError> {
        let (input, value) = le_f32(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_bool(&mut self) -> Result<bool, ParseError> {
        let (input, value) = le_u8(self.input)?;
        self.input = input;
        Ok(value > 0)
    }

    fn next_encoded_u32(&mut self) -> Result<u32, ParseError> {
        let (input, value) = parse_encode_u32(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_encoded_i32(&mut self) -> Result<i32, ParseError> {
        let (input, value) = parse_encode_i32(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_encoded_u64(&mut self) -> Result<u64, ParseError> {
        let (input, value) = parse_encode_u64(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_encoded_i64(&mut self) -> Result<i64, ParseError> {
        let (input, value) = parse_encode_i64(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_enum<T>(&mut self) -> Result<T, ParseError>
    where
        T: From<u8> + Debug,
    {
        let (input, value) = parse_enum(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_string(&mut self) -> Result<String, ParseError> {
        let (input, value) = parse_string(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_bytes(&mut self, count: usize) -> Result<&[u8], ParseError> {
        let (input, data) = take(count)(self.input)?;
        self.input = input;
        Ok(data)
    }

    fn next_tag_block(&mut self, ctx: impl ParserContext) -> Result<TagBlock, ParseError> {
        let mut block = TagBlock { tags: vec![] };
        loop {
            let tag = Tag::parse_b(self, ctx.clone())?;
            match tag.header.code {
                TagCode::End => {
                    return Ok(block);
                }
                _ => block.tags.push(tag),
            }
        }
    }

    // fn parse<T>(&mut self) -> Result<T, ParseError>
    // where
    //     T: Parsable,
    // {
    //     let (input, value) = T::parse(self)?;
    //     self.input = input;
    //     Ok(value)
    // }
}

#[derive(Debug)]
pub struct PagParser<'a> {
    header: FileHeader,
    inner: SliceParser<'a>,
}

impl<'a> PagParser<'a> {
    const DEFAULT_PAG_VERSION: u8 = 1;

    pub fn new(input: &'a [u8]) -> Result<Self, ParseError> {
        let (input, header) = FileHeader::parse(input)?;
        if header.version != Self::DEFAULT_PAG_VERSION {
            return Err(ParseError::UnsupportPagVersion(header.version));
        }
        Ok(Self {
            header,
            inner: SliceParser { input },
        })
    }
}

impl<'a> PagParser<'a> {
    // fn parse_with<T>(&mut self, ctx: impl ParserContext) -> Result<T, ParseError>
    // where
    //     T: ContextualParsable,
    // {
    //     if self.input.is_empty() {
    //         return Err(ParseError::Eof);
    //     }

    //     let (input, value) = T::parse_with(self.input, ctx)?;
    //     self.input = input;
    //     Ok(value)
    // }

    pub fn next_tag(&mut self) -> Result<Tag, ParseError> {
        Tag::parse_b(&mut self.inner, ())
    }

    // pub fn parse_all(&mut self) -> Result<TagBlock, ParseError> {
    //     let (input, result) = TagBlock::parse(self.input)?;
    //     self.input = input;
    //     Ok(result)
    // }
}

#[cfg(test)]
mod tests {
    use std::{fs, io};

    use nom::Err;

    use crate::format::{FileHeader, StreamParser, TagBlock};

    use super::{PagParser, ParseError};

    #[test]
    fn test_parse_pag() -> Result<(), ParseError> {
        let _ = env_logger::builder()
            .format_module_path(false)
            .filter_level(log::LevelFilter::Debug)
            .try_init();

        let pag = fs::read("libpag/resources/apitest/complex_test.pag")?;
        log::info!("full length = {} bytes", pag.len());

        let mut parser = PagParser::new(pag.as_slice())?;
        log::info!("header = {:?}", parser.header);
        log::info!("====================");

        while let Ok(tag) = parser.next_tag() {
            log::info!("{:?}", tag);
            log::info!("====================");
        }

        Ok(())
    }

    // #[test]
    fn test_parse_pag_raw() -> Result<(), ParseError> {
        let _ = env_logger::builder()
            .format_module_path(false)
            .filter_level(log::LevelFilter::Debug)
            .try_init();

        let pag = fs::read("libpag/resources/apitest/complex_test.pag")?;
        println!("full length = {} bytes", pag.len());

        let (input, header) = FileHeader::parse(pag.as_slice())
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "parse_file_header"))?;
        println!("header = {:?}", header);

        let (input, tag_block) = TagBlock::parse(input).map_err(|e| {
            match e {
                Err::Incomplete(error) => {
                    log::error!("Incomplete: {:?}", error);
                }
                Err::Error(error) => {
                    log::error!("Error: {:?}", error.code);
                }
                Err::Failure(error) => {
                    log::error!("Failure: {:?}", error.code);
                }
            };
            io::Error::new(io::ErrorKind::Other, "parse_tag_block")
        })?;
        println!("tag_block = {:?}", tag_block);
        println!("remain = {:?} bytes", input);

        Ok(())
    }
}
