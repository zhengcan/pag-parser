use crate::format::ByteData;

use super::{shape::CompositionAttributes, AttributeBlock, TagBlock};

/// VideoCompositionBlock 存储了 1 个或多个不同尺⼨的视频序列帧。
#[derive(Debug)]
pub struct VideoCompositionBlock {
    pub id: u32,
    pub has_alpha: bool,
    pub composition_attributes: CompositionAttributes,
    pub tag_block: TagBlock,
}

/// VideoSequence 存储了 1 个版本的视频序列帧的结构。
#[derive(Debug)]
pub struct VideoSequence {
    pub inner: AttributeBlock,
    pub width: u32,
    pub height: u32,
    pub frame_rate: f32,
    pub alpha_start_x: i32,
    pub alpha_start_y: i32,
    pub sps_data: ByteData,
    pub pps_data: ByteData,
    pub frame_count: u32,
    pub is_key_frame_flag: Vec<bool>,
    pub video_frames: Vec<u32>,
}
