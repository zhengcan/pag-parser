use std::marker::PhantomData;

use nom::{number::complete::le_f32, sequence::tuple};

use crate::format::{parse_encode_i32, parse_time, Bits};

use super::{
    primitive::{parse_bool, parse_encode_u32},
    ByteData, ParserContext, StreamParser, TagBlock, Time,
};

/// VideoCompositionBlock 存储了 1 个或多个不同尺⼨的视频序列帧。
#[derive(Debug)]
pub struct VideoCompositionBlock {
    pub id: u32,
    pub has_alpha: bool,
    pub tag_block: TagBlock,
}

impl StreamParser for VideoCompositionBlock {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        log::debug!(
            "parse_VideoCompositionBlock <= {} bytes: {:?}",
            input.len(),
            &input[0..16]
        );
        let (input, (id, has_alpha)) = tuple((parse_encode_u32, parse_bool))(input)?;
        // log::warn!("{id},{has_alpha}");
        let (input, tag_block) = TagBlock::parse_with(input, has_alpha)?;
        let result = Self {
            id,
            has_alpha,
            tag_block,
        };
        log::debug!("parse_VideoCompositionBlock => {:?}", result);
        Ok((input, result))
    }
}

/// VideoSequence 存储了 1 个版本的视频序列帧的结构。
#[derive(Debug)]
pub struct VideoSequence {
    pub width: i32,
    pub height: i32,
    pub frame_rate: f32,
    pub alpha_start_x: Option<i32>,
    pub alpha_start_y: Option<i32>,
    pub sps_data: ByteData,
    pub pps_data: ByteData,
    pub frame_count: u32,
    pub is_key_frame_flag: Vec<bool>,
    pub video_frames: Vec<VideoFrame>,
    pub static_time_ranges: Vec<TimeRange>,
}

impl StreamParser for VideoSequence {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        Self::parse_with(input, ())
    }

    fn parse_with(input: &[u8], ctx: impl ParserContext) -> nom::IResult<&[u8], Self> {
        log::debug!(
            "parse_VideoSequence <= {} bytes: {:?}",
            input.len(),
            &input[0..16]
        );

        let (mut input, (width, height, frame_rate)) =
            tuple((parse_encode_i32, parse_encode_i32, le_f32))(input)?;
        let (alpha_start_x, alpha_start_y) = match ctx.as_bool() {
            true => {
                let (next, (alpha_start_x, alpha_start_y)) =
                    tuple((parse_encode_i32, parse_encode_i32))(input)?;
                input = next;
                (Some(alpha_start_x), Some(alpha_start_y))
            }
            false => (None, None),
        };
        // log::warn!(
        //     "{},{width},{height},{frame_rate},{alpha_start_x:?},{alpha_start_y:?}",
        //     ctx.as_bool()
        // );
        let (input, (sps_data, pps_data, frame_count)) =
            tuple((ByteData::parse, ByteData::parse, parse_encode_u32))(input)?;
        // log::warn!("{frame_count}",);

        let mut bits = Bits::new(input);
        // log::warn!("{:?}", &input[..8]);
        let mut is_key_frame_flag = vec![];
        for _ in 0..frame_count {
            is_key_frame_flag.push(bits.next());
        }
        let mut input = bits.finish();
        // log::warn!("{:?}", &input[..8]);

        let mut video_frames = vec![];
        for i in 0..frame_count {
            let (next, frame) = VideoFrame::parse_and(input, |mut frame| {
                frame.is_key_frame = is_key_frame_flag
                    .get(i as usize)
                    .map(|v| *v)
                    .unwrap_or_default();
            })?;
            input = next;
            video_frames.push(frame);
        }

        let mut static_time_ranges = vec![];
        if input.len() > 0 {
            let (next, count) = parse_encode_u32(input)?;
            input = next;
            for _ in 0..count {
                let (next, time_range) = TimeRange::parse(input)?;
                input = next;
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
        Ok((input, result))
    }
}

#[derive(Debug)]
pub struct TimeRange {
    pub start: Time,
    pub end: Time,
}

impl StreamParser for TimeRange {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        let (input, (start, end)) = tuple((parse_time, parse_time))(input)?;
        let result = Self { start, end };
        Ok((input, result))
    }
}

/// 视频帧信息。
#[derive(Debug)]
pub struct VideoFrame {
    pub is_key_frame: bool,
    pub frame: Time,
    pub file_bytes: ByteData,
}

impl StreamParser for VideoFrame {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        Self::parse_and(input, |_| {})
    }

    fn parse_and<F>(input: &[u8], f: F) -> nom::IResult<&[u8], Self>
    where
        F: Fn(&mut Self),
    {
        log::debug!(
            "parse_VideoFrame <= {} bytes: {:?}",
            input.len(),
            &input[0..16]
        );

        let (input, (frame, file_bytes)) = tuple((parse_time, ByteData::parse))(input)?;
        let mut result = Self {
            is_key_frame: false,
            frame,
            file_bytes,
        };

        f(&mut result);

        log::debug!("parse_VideoFrame => {:?}", result);
        Ok((input, result))
    }
}
