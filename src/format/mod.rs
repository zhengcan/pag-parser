mod base;
mod block;
mod image;
mod layer;
mod shape;
mod text;
mod video;

pub use base::*;
pub use block::*;
pub use image::*;
pub use layer::*;
pub use shape::*;
pub use text::*;
pub use video::*;

/// Pag 文件格式
#[derive(Debug)]
pub struct Pag {
    pub header: FileHeader,
    pub tag_block: TagBlock,
}

/// Pag 文件头
#[derive(Debug)]
pub struct FileHeader {
    pub magic: [u8; 3],
    pub version: u8,
    pub length: u32,
    pub compress_method: i8,
}
