use crate::parse::{EncodedInt32, EncodedUint32, Parsable, ParseContext, ParseError, Parser};

use super::{ByteData, TagBlock};

/// ImageTables 是图⽚信息的合集。
#[derive(Debug)]
pub struct ImageTables {
    pub count: EncodedInt32,
    pub images: Vec<ImageBytes>,
}

impl Parsable for ImageTables {
    fn parse(parser: &mut impl Parser, ctx: impl ParseContext) -> Result<Self, ParseError> {
        let count = parser.next_encoded_i32()?;
        let mut images = vec![];
        for _ in 0..count.to_i32() {
            let image = ImageBytes::parse(parser, ctx.clone())?;
            images.push(image);
        }
        let result = Self { count, images };
        log::debug!("parse_ImageTables => {:?}", result);
        Ok(result)
    }
}

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
    pub id: EncodedUint32,
}

impl Parsable for ImageReference {
    fn parse(parser: &mut impl Parser, _ctx: impl ParseContext) -> Result<Self, ParseError> {
        let id = parser.next_id()?;
        let result = Self { id };
        log::debug!("parse_ImageReference => {:?}", result);
        Ok(result)
    }
}

/// ImageBytes 图⽚标签，存储了压缩后的图⽚相关属性信息。
#[derive(Debug)]
pub struct ImageBytes {
    pub id: EncodedUint32,
    pub file_bytes: ByteData,
}

impl Parsable for ImageBytes {
    fn parse(parser: &mut impl Parser, ctx: impl ParseContext) -> Result<Self, ParseError> {
        let id = parser.next_encoded_u32()?;
        let file_bytes = ByteData::parse(parser, ctx)?;
        let result = Self { id, file_bytes };
        log::debug!("parse_ImageBytes => {:?}", result);
        Ok(result)
    }
}

/// ImageBytes2 图⽚标签版本 2，除了存储 ImageBytes 的信息外，还允许记录图⽚的缩放参数，通常根据实际最⼤⽤到的⼤⼩来存储图⽚，⽽不是按原始⼤⼩。
#[derive(Debug)]
pub struct ImageBytes2 {
    pub id: EncodedUint32,
    pub file_bytes: ByteData,
    pub scale_factor: f32,
}

impl Parsable for ImageBytes2 {
    fn parse(parser: &mut impl Parser, ctx: impl ParseContext) -> Result<Self, ParseError> {
        let id = parser.next_encoded_u32()?;
        let file_bytes = ByteData::parse(parser, ctx)?;
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

/// ImageBytes3 图⽚标签版本 3， 除了包含 ImageBytes2 的信息外，还允许记录剔除透明边框后的图⽚。
#[derive(Debug)]
pub struct ImageBytes3 {
    pub id: EncodedUint32,
    pub file_bytes: ByteData,
    pub scale_factor: f32,
    pub width: EncodedInt32,
    pub height: EncodedInt32,
    pub anchor_x: EncodedInt32,
    pub anchor_y: EncodedInt32,
}

impl Parsable for ImageBytes3 {
    fn parse(parser: &mut impl Parser, ctx: impl ParseContext) -> Result<Self, ParseError> {
        let id = parser.next_encoded_u32()?;
        let file_bytes = ByteData::parse(parser, ctx)?;
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
