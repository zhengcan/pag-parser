use crate::format::ByteData;

use super::TagBlock;

#[derive(Debug)]
pub struct BitmapRect {}

/// ImageTables 是图⽚信息的合集。
#[derive(Debug)]
pub struct ImageTables {
    pub count: i32,
    pub images: Vec<ImageBytes>,
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

/// ImageReference 图⽚引⽤标签，存储的是⼀个图⽚的唯⼀ ID，通过 ID 索引真正的图⽚信息。
#[derive(Debug)]
pub struct ImageReference {
    // pub inner: AttributeBlock,
    pub id: u32,
}

/// ImageBytes 图⽚标签，存储了压缩后的图⽚相关属性信息。
#[derive(Debug)]
pub struct ImageBytes {
    pub id: u32,
    pub file_bytes: ByteData,
}

/// ImageBytes2 图⽚标签版本 2，除了存储 ImageBytes 的信息外，还允许记录图⽚的缩放参数，通常根据实际最⼤⽤到的⼤⼩来存储图⽚，⽽不是按原始⼤⼩。
#[derive(Debug)]
pub struct ImageBytes2 {
    pub id: u32,
    pub file_bytes: ByteData,
    pub scale_factor: f32,
}

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
