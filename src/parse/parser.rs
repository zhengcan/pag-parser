use std::cmp::min;
use std::fmt::Debug;

use nom::{
    bytes::complete::{tag, take, take_until},
    number::complete::{le_f32, le_i32, le_i64, le_i8, le_u16, le_u32, le_u64, le_u8},
};

use super::{
    attr::AttributeBlock,
    bits::Bits,
    types::{EncodedInt32, EncodedInt64, EncodedUint32, EncodedUint64, Time},
    ParseError, Parsable,
};

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

    fn next_term<'b, 'c>(&'b mut self, tag: &'c str) -> Result<&'b [u8], ParseError>;

    fn next_u8(&mut self) -> Result<u8, ParseError>;
    fn next_i8(&mut self) -> Result<i8, ParseError>;
    fn next_u16(&mut self) -> Result<u16, ParseError>;
    fn next_u32(&mut self) -> Result<u32, ParseError>;
    fn next_i32(&mut self) -> Result<i32, ParseError>;
    fn next_u64(&mut self) -> Result<u64, ParseError>;
    fn next_i64(&mut self) -> Result<i64, ParseError>;
    fn next_f32(&mut self) -> Result<f32, ParseError>;
    fn next_bool(&mut self) -> Result<bool, ParseError>;

    fn next_encoded_u32(&mut self) -> Result<EncodedUint32, ParseError>;
    fn next_encoded_i32(&mut self) -> Result<EncodedInt32, ParseError>;
    fn next_encoded_u64(&mut self) -> Result<EncodedUint64, ParseError>;
    fn next_encoded_i64(&mut self) -> Result<EncodedInt64, ParseError>;

    fn next_enum<T>(&mut self) -> Result<T, ParseError>
    where
        T: From<u8> + Debug;
    fn next_string(&mut self) -> Result<String, ParseError>;
    fn next_bytes(&mut self, count: usize) -> Result<&[u8], ParseError>;

    #[inline(always)]
    fn next_id(&mut self) -> Result<EncodedUint32, ParseError> {
        self.next_encoded_u32()
    }
    #[inline(always)]
    fn next_time(&mut self) -> Result<Time, ParseError> {
        self.next_encoded_u64()
    }

    #[inline(always)]
    fn next<T>(&mut self) -> Result<T, ParseError>
    where
        T: Parsable,
        Self: Sized,
    {
        T::parse(self, ())
    }
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

    fn next_term<'b, 'c>(&'b mut self, term: &'c str) -> Result<&'b [u8], ParseError> {
        let (input, term) = tag(term)(self.input)?;
        self.input = input;
        Ok(term)
    }

    fn next_u8(&mut self) -> Result<u8, ParseError> {
        let (input, value) = le_u8(self.input)?;
        self.input = input;
        Ok(value)
    }

    fn next_i8(&mut self) -> Result<i8, ParseError> {
        let (input, value) = le_i8(self.input)?;
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

    fn next_encoded_u32(&mut self) -> Result<EncodedUint32, ParseError> {
        let mut input = self.input;
        let mut value = 0u32;
        for i in (0..32).step_by(7) {
            let (next, byte) = le_u8(input)?;
            input = next;
            value |= ((byte & 0x7f) as u32) << i;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        self.input = input;
        Ok(EncodedUint32::from(value))
    }

    fn next_encoded_i32(&mut self) -> Result<EncodedInt32, ParseError> {
        let value = self.next_encoded_u32()?.to_u32();
        let num = (value >> 1) as i32;
        let value = match (value & 1) > 0 {
            true => -num,
            false => num,
        };
        Ok(EncodedInt32::from(value))
    }

    fn next_encoded_u64(&mut self) -> Result<EncodedUint64, ParseError> {
        let mut input = self.input;
        let mut value = 0u64;
        for i in (0..64).step_by(7) {
            let (next, byte) = le_u8(input)?;
            input = next;
            value |= ((byte & 0x7f) as u64) << i;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        self.input = input;
        Ok(EncodedUint64::from(value))
    }

    fn next_encoded_i64(&mut self) -> Result<EncodedInt64, ParseError> {
        let value = self.next_encoded_u64()?.to_u64();
        let num = (value >> 1) as i64;
        let value = match (value & 1) > 0 {
            true => -num,
            false => num,
        };
        Ok(EncodedInt64::from(value))
    }

    fn next_enum<T>(&mut self) -> Result<T, ParseError>
    where
        T: From<u8> + Debug,
    {
        let (input, value) = le_u8(self.input)?;
        self.input = input;
        Ok(T::from(value))
    }

    fn next_string(&mut self) -> Result<String, ParseError> {
        let (input, value) = take_until("\0")(self.input)?;
        self.input = input;
        Ok(String::from_utf8_lossy(value).to_string())
    }

    fn next_bytes(&mut self, count: usize) -> Result<&[u8], ParseError> {
        let (input, data) = take(count)(self.input)?;
        self.input = input;
        Ok(data)
    }

    // fn next_tag_block(&mut self, ctx: impl ParserContext) -> Result<TagBlock, ParseError> {
    //     let mut block = TagBlock { tags: vec![] };
    //     loop {
    //         let tag = Tag::parse_b(self, ctx.clone())?;
    //         match tag.header.code {
    //             TagCode::End => {
    //                 return Ok(block);
    //             }
    //             _ => block.tags.push(tag),
    //         }
    //     }
    // }

    // fn parse<T>(&mut self) -> Result<T, ParseError>
    // where
    //     T: Parsable,
    // {
    //     let (input, value) = T::parse(self)?;
    //     self.input = input;
    //     Ok(value)
    // }
}
