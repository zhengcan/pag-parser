use crate::parse::{
    EncodedInt32, EncodedUint32, Parsable, ParseError, Parser, ParserContext, Time,
};

use super::{ByteData, TagBlock};

/// VideoCompositionBlock 存储了 1 个或多个不同尺⼨的视频序列帧。
#[derive(Debug)]
pub struct VideoCompositionBlock {
    pub id: EncodedUint32,
    pub has_alpha: bool,
    pub tag_block: TagBlock,
}

impl Parsable for VideoCompositionBlock {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let id = parser.next_id()?;
        let has_alpha = parser.next_bool()?;
        let tag_block = TagBlock::parse(parser, ctx)?;
        let result = Self {
            id,
            has_alpha,
            tag_block,
        };
        log::debug!("parse_VideoCompositionBlock => {:?}", result);
        Ok(result)
    }
}

/// VideoSequence 存储了 1 个版本的视频序列帧的结构。
#[derive(Debug)]
pub struct VideoSequence {
    pub width: EncodedInt32,
    pub height: EncodedInt32,
    pub frame_rate: f32,
    pub alpha_start_x: Option<EncodedInt32>,
    pub alpha_start_y: Option<EncodedInt32>,
    pub sps_data: ByteData,
    pub pps_data: ByteData,
    pub frame_count: EncodedUint32,
    pub is_key_frame_flag: Vec<bool>,
    pub video_frames: Vec<VideoFrame>,
    pub static_time_ranges: Vec<TimeRange>,
}

impl Parsable for VideoSequence {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let width = parser.next_encoded_i32()?;
        let height = parser.next_encoded_i32()?;
        let frame_rate = parser.next_f32()?;
        let (alpha_start_x, alpha_start_y) = match ctx.as_bool() {
            true => {
                let alpha_start_x = parser.next_encoded_i32()?;
                let alpha_start_y = parser.next_encoded_i32()?;
                (Some(alpha_start_x), Some(alpha_start_y))
            }
            false => (None, None),
        };
        let sps_data = ByteData::parse(parser, ctx.clone())?;
        let pps_data = ByteData::parse(parser, ctx.clone())?;
        let frame_count = parser.next_encoded_u32()?;

        let mut bits = parser.new_bits();
        // log::warn!("{:?}", &input[..8]);
        let mut is_key_frame_flag = vec![];
        for _ in 0..frame_count.to_u32() {
            is_key_frame_flag.push(bits.next());
        }
        // let mut input = bits.finish();
        // log::warn!("{:?}", &input[..8]);
        let parser = &mut bits.finish();

        let mut video_frames = vec![];
        for i in 0..frame_count.to_u32() {
            let mut frame = VideoFrame::parse(parser, ctx.clone())?;
            frame.is_key_frame = is_key_frame_flag
                .get(i as usize)
                .map(|v| *v)
                .unwrap_or_default();
            video_frames.push(frame);
        }

        let mut static_time_ranges = vec![];
        if parser.remain() > 0 {
            let count = parser.next_encoded_u32()?;
            for _ in 0..count.to_u32() {
                let time_range = TimeRange::parse(parser, ctx.clone())?;
                static_time_ranges.push(time_range);
            }
        }

        let result = Self {
            width,
            height,
            frame_rate,
            alpha_start_x,
            alpha_start_y,
            sps_data,
            pps_data,
            frame_count,
            is_key_frame_flag,
            video_frames,
            static_time_ranges,
        };

        log::debug!("parse_VideoSequence => {:?}", result);
        Ok(result)
    }
}

#[derive(Debug)]
pub struct TimeRange {
    pub start: Time,
    pub end: Time,
}

impl Parsable for TimeRange {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let start = parser.next_time()?;
        let end = parser.next_time()?;
        Ok(Self { start, end })
    }
}

/// 视频帧信息。
#[derive(Debug)]
pub struct VideoFrame {
    pub is_key_frame: bool,
    pub frame: Time,
    pub file_bytes: ByteData,
}

impl Parsable for VideoFrame {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let frame = parser.next_time()?;
        let file_bytes = ByteData::parse(parser, ctx)?;
        let result = Self {
            is_key_frame: false,
            frame,
            file_bytes,
        };
        log::debug!("parse_VideoFrame => {:?}", result);
        Ok(result)
    }
}
