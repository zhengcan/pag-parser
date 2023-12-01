use nom::{number::complete::le_f32, sequence::tuple, IResult};

use crate::{
    format::primitive::{parse_encode_i32, parse_encode_u32, parse_encode_u64},
    parser::{ParseError, Parser},
};

use super::{
    BlendMode, Color, CompositeOrder, ContextualParsable, FillRule, GradientFillType, LineCap,
    LineJoin, MergePathsMode, ParserContext, Path, Point, StreamParser, TagBlock, TrimPathsType,
};

/// VectorCompositionBlock 是⽮量图形的合集。⾥⾯可以包含简单的⽮量图形，也可以再包含⼀个或是多个 VectorComposition。
#[derive(Debug)]
pub struct VectorCompositionBlock {
    pub id: u32,
    pub tag_block: TagBlock,
}

impl ContextualParsable for VectorCompositionBlock {
    fn parse_b(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let id = parser.next_id()?;
        let tag_block = parser.next_tag_block(ctx)?;
        let result = Self { id, tag_block };
        log::debug!("parse_VectorCompositionBlock => {:?}", result);
        Ok(result)
    }
}

impl StreamParser for VectorCompositionBlock {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_VectorCompositionBlock <= {} bytes", input.len());
        let (input, (id, tag_block)) = tuple((parse_encode_u32, TagBlock::parse))(input)?;
        let result = Self { id, tag_block };
        log::debug!("parse_VectorCompositionBlock => {:?}", result);
        Ok((input, result))
    }
}

/// CompositionAttribute 存储了 Composition 基本属性信息。⾥⾯可以包含简单的⽮量图形，也可以再包含⼀个或是多个 VectorComposition。
#[derive(Debug)]
pub struct CompositionAttributes {
    pub width: i32,
    pub height: i32,
    pub duration: u64,
    pub frame_rate: f32,
    pub background_color: Color,
}

impl ContextualParsable for CompositionAttributes {
    fn parse_b(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let width = parser.next_encoded_i32()?;
        let height = parser.next_encoded_i32()?;
        let duration = parser.next_encoded_u64()?;
        let frame_rate = parser.next_f32()?;
        let background_color = parser.next_color()?;

        let result = Self {
            width,
            height,
            duration,
            frame_rate,
            background_color,
        };

        log::debug!("parse_CompositionAttributes => {:?}", result);
        Ok(result)
    }
}

// impl StreamParser for CompositionAttributes {
//     fn parse(input: &[u8]) -> IResult<&[u8], Self> {
//         log::debug!(
//             "parse_CompositionAttributes <= {} bytes: {:?}",
//             input.len(),
//             &input[0..8]
//         );

//         let (input, (width, height, duration, frame_rate, background_color)) = tuple((
//             parse_encode_i32,
//             parse_encode_i32,
//             parse_encode_u64,
//             le_f32,
//             Color::parse,
//         ))(input)?;
//         let result = Self {
//             width,
//             height,
//             duration,
//             frame_rate,
//             background_color,
//         };

//         log::debug!("parse_CompositionAttributes => {:?}", result);
//         Ok((input, result))
//     }
// }

/// ShapeGroup 投影标签。
#[derive(Debug)]
pub struct ShapeGroup {
    pub blend_mode: BlendMode,
    pub anchor_point: Point,
    pub position: Point,
    pub scale: Point,
    pub skew: f32,
    pub skew_axis: f32,
    pub rotation: f32,
    pub opacity: u8,
    pub tag_block: TagBlock,
}

/// 矩形标签。
#[derive(Debug)]
pub struct Rectangle {
    pub reversed: bool,
    pub size: Point,
    pub position: Point,
    pub roundness: f32,
}

/// Ellipse 标签。
#[derive(Debug)]
pub struct Ellipse {
    pub reversed: bool,
    pub size: Point,
    pub position: Point,
}

/// 多边星形标签。
#[derive(Debug)]
pub struct PolyStar {
    pub reversed: bool,
    pub poly_type: u8,
    pub points: f32,
    pub position: Point,
    pub rotate: f32,
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub inner_roundness: f32,
    pub outer_roundness: f32,
}

/// ShapePath 标签。
#[derive(Debug)]
pub struct ShapePath {
    pub shape_path: Path,
}

/// Fill 标签。
#[derive(Debug)]
pub struct Fill {
    pub blend_mode: BlendMode,
    pub composite: CompositeOrder,
    pub fill_rule: FillRule,
    pub color: Color,
    pub opacity: u8,
}

/// Stroke 标签。
#[derive(Debug)]
pub struct Stroke {
    pub blend_mode: BlendMode,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub miter_limit: f32,
    pub color: Color,
    pub opacity: u8,
    pub stroke_width: f32,
}

/// GradientFill 标签。
#[derive(Debug)]
pub struct GradientFill {
    pub blend_mode: BlendMode,
    pub composite: CompositeOrder,
    pub fill_rule: FillRule,
    pub fill_type: GradientFillType,
    pub start_point: Point,
    pub end_point: Point,
    pub colors: Vec<Color>,
    pub opacity: u8,
}

/// GradientStroke 标签。
#[derive(Debug)]
pub struct GradientStroke {
    pub blend_mode: BlendMode,
    pub composite: CompositeOrder,
    pub fill_type: GradientFillType,
    pub start_point: Point,
    pub end_point: Point,
    pub color: Color,
    pub opacity: u8,
    pub stroke_width: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub miter_limit: f32,
    pub dash_length: [u32; 3],
    pub dash_offset_flag_exist: [bool; 1],
    pub dash_offset_flag_animatable: [bool; 1],
}

/// MergePaths 标签。
#[derive(Debug)]
pub struct MergePaths {
    pub mode: MergePathsMode,
}

/// TrimPaths 标签。
#[derive(Debug)]
pub struct TrimPaths {
    pub start: f32,
    pub end: f32,
    pub offset: f32,
    pub trim_type: TrimPathsType,
}

/// RoundCorners 标签。
#[derive(Debug)]
pub struct RoundCorners {
    pub radius: f32,
}
