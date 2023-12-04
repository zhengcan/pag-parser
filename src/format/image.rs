use crate::parser::{ParseError, Parser};

use super::{ByteData, ContextualParsable, Parsable, ParserContext, TagBlock};

/// ImageTables 是图⽚信息的合集。
#[derive(Debug)]
pub struct ImageTables {
    pub count: i32,
    pub images: Vec<ImageBytes>,
}

impl ContextualParsable for ImageTables {
    fn parse_b(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let count = parser.next_encoded_i32()?;
        let mut images = vec![];
        for _ in 0..count {
            let image = ImageBytes::parse(parser)?;
            images.push(image);
        }
        let result = Self { count, images };
        log::debug!("parse_ImageTables => {:?}", result);
        Ok(result)
    }
}

// impl StreamParser for ImageTables {
//     fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
//         log::debug!("parse_ImageTables <= {} bytes", input.len());
//         let (mut input, count) = parse_encode_i32(input)?;
//         let mut images = vec![];
//         for _ in 0..count {
//             let (next, image) = ImageBytes::parse(input)?;
//             input = next;
//             images.push(image);
//         }
//         let result = Self { count, images };
//         log::debug!("parse_ImageTables => {:?}", result);
//         Ok((input, result))
//     }
// }

/// BitmapCompositionBlock 位图序列帧标签。
#[derive(Debug)]
pub struct BitmapCompositionBlock {
    // pub inner: AttributeBlock,
    pub id: u32,
    pub tag_block: TagBlock,
}

/// BitmapSequence 标签。
#[derive(Debug)]
pub struct BitmapSequence {
    // pub inner: AttributeBlock,
    pub width: u32,
    pub height: u32,
    pub frame_rate: f32,
    pub frame_count: u32,
    pub is_key_frame_flag: Vec<bool>,
    pub bitmap_rect: Vec<BitmapRect>,
}

#[derive(Debug)]
pub struct BitmapRect {}

/// ImageReference 图⽚引⽤标签，存储的是⼀个图⽚的唯⼀ ID，通过 ID 索引真正的图⽚信息。
#[derive(Debug)]
pub struct ImageReference {
    // pub inner: AttributeBlock,
    pub id: u32,
}

impl ContextualParsable for ImageReference {
    fn parse_b(parser: &mut impl Parser, _ctx: impl ParserContext) -> Result<Self, ParseError> {
        let id = parser.next_id()?;
        let result = Self { id };
        log::debug!("parse_ImageReference => {:?}", result);
        Ok(result)
    }
}

// impl StreamParser for ImageReference {
//     fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
//         log::debug!("parse_ImageReference <= {} bytes", input.len());
//         let (input, id) = parse_encode_u32(input)?;
//         let result = Self { id };
//         log::debug!("parse_ImageReference => {:?}", result);
//         Ok((input, result))
//     }
// }

/// ImageBytes 图⽚标签，存储了压缩后的图⽚相关属性信息。
#[derive(Debug)]
pub struct ImageBytes {
    pub id: u32,
    pub file_bytes: ByteData,
}

impl Parsable for ImageBytes {
    fn parse(parser: &mut impl Parser) -> Result<Self, ParseError> {
        let id = parser.next_encoded_u32()?;
        let file_bytes = ByteData::parse(parser)?;
        let result = Self { id, file_bytes };
        log::debug!("parse_ImageBytes => {:?}", result);
        Ok(result)
    }
}

impl ContextualParsable for ImageBytes {
    fn parse_b(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        Self::parse(parser)
    }
}

// impl StreamParser for ImageBytes {
//     fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
//         log::debug!("parse_ImageBytes <= {} bytes", input.len());
//         let (input, (id, file_bytes)) = tuple((parse_encode_u32, ByteData::parse))(input)?;
//         let result = Self { id, file_bytes };
//         log::debug!("parse_ImageBytes => {:?}", result);
//         Ok((input, result))
//     }
// }

/// ImageBytes2 图⽚标签版本 2，除了存储 ImageBytes 的信息外，还允许记录图⽚的缩放参数，通常根据实际最⼤⽤到的⼤⼩来存储图⽚，⽽不是按原始⼤⼩。
#[derive(Debug)]
pub struct ImageBytes2 {
    pub id: u32,
    pub file_bytes: ByteData,
    pub scale_factor: f32,
}

impl ContextualParsable for ImageBytes2 {
    fn parse_b(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let id = parser.next_encoded_u32()?;
        let file_bytes = ByteData::parse(parser)?;
        let scale_factor = parser.next_f32()?;
        let result = Self {
            id,
            file_bytes,
            scale_factor,
        };
        log::debug!("parse_ImageBytes2 => {:?}", result);
        Ok(result)
    }
}

// impl StreamParser for ImageBytes2 {
//     fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
//         log::debug!("parse_ImageBytes2 <= {} bytes", input.len());
//         let (input, (id, file_bytes, scale_factor)) =
//             tuple((parse_encode_u32, ByteData::parse, le_f32))(input)?;
//         let result = Self {
//             id,
//             file_bytes,
//             scale_factor,
//         };
//         log::debug!("parse_ImageBytes2 => {:?}", result);
//         Ok((input, result))
//     }
// }

/// ImageBytes3 图⽚标签版本 3， 除了包含 ImageBytes2 的信息外，还允许记录剔除透明边框后的图⽚。
#[derive(Debug)]
pub struct ImageBytes3 {
    pub id: u32,
    pub file_bytes: ByteData,
    pub scale_factor: f32,
    pub width: i32,
    pub height: i32,
    pub anchor_x: i32,
    pub anchor_y: i32,
}

impl ContextualParsable for ImageBytes3 {
    fn parse_b(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let id = parser.next_encoded_u32()?;
        let file_bytes = ByteData::parse(parser)?;
        let scale_factor = parser.next_f32()?;
        let width = parser.next_encoded_i32()?;
        let height = parser.next_encoded_i32()?;
        let anchor_x = parser.next_encoded_i32()?;
        let anchor_y = parser.next_encoded_i32()?;
        let result = Self {
            id,
            file_bytes,
            scale_factor,
            width,
            height,
            anchor_x,
            anchor_y,
        };
        log::debug!("parse_ImageBytes3 => {:?}", result);
        Ok(result)
    }
}

// impl StreamParser for ImageBytes3 {
//     fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
//         log::debug!("parse_ImageBytes3 <= {} bytes", input.len());
//         let (input, (id, file_bytes, scale_factor, width, height, anchor_x, anchor_y)) =
//             tuple((
//                 parse_encode_u32,
//                 ByteData::parse,
//                 le_f32,
//                 parse_encode_i32,
//                 parse_encode_i32,
//                 parse_encode_i32,
//                 parse_encode_i32,
//             ))(input)?;
//         let result = Self {
//             id,
//             file_bytes,
//             scale_factor,
//             width,
//             height,
//             anchor_x,
//             anchor_y,
//         };
//         log::debug!("parse_ImageBytes3 => {:?}", result);
//         Ok((input, result))
//     }
// }
