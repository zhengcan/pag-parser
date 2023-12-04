use std::fmt::Debug;

use macros::ParsableEnum;
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::parse::{
    AttributeValue, EncodedInt32, EncodedUint32, Parsable, ParseError, Parser, ParserContext,
};

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Parsable for Color {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let red = parser.next_u8()?;
        let green = parser.next_u8()?;
        let blue = parser.next_u8()?;
        Ok(Self { red, green, blue })
    }
}

pub struct ByteData {
    pub length: EncodedUint32,
    pub data: Vec<u8>,
}

impl ByteData {
    pub fn from(data: &[u8]) -> Self {
        Self {
            length: EncodedUint32::from(data.len() as u32),
            data: Vec::from(data),
        }
    }
}

impl Debug for ByteData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ByteData").field(&self.length).finish()
    }
}

impl Parsable for ByteData {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let length = parser.next_encoded_u32()?;
        let data = parser.next_bytes(length.to_usize())?;
        // let (input, data) = take(length)(input)?;
        assert_eq!(length, data.len() as u32);
        let result = Self {
            length,
            data: Vec::from(data),
        };
        log::debug!("parse_ByteData => {:?}", result);
        Ok(result)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, ParsableEnum)]
#[repr(u8)]
pub enum TrimPathsType {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, ParsableEnum)]
#[repr(u8)]
pub enum MergePathsMode {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, ParsableEnum)]
#[repr(u8)]
pub enum GradientFillType {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, ParsableEnum)]
#[repr(u8)]
pub enum LineCap {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, ParsableEnum)]
#[repr(u8)]
pub enum LineJoin {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, ParsableEnum)]
#[repr(u8)]
pub enum CompositeOrder {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, ParsableEnum)]
#[repr(u8)]
pub enum FillRule {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, ParsableEnum)]
#[repr(u8)]
pub enum MaskMode {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug)]
pub struct Path {}

impl Parsable for Path {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        Ok(Self {})
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0., y: 0. }
    }

    pub fn one() -> Self {
        Self { x: 1., y: 1. }
    }
}

impl AttributeValue for Point {}

impl Parsable for Point {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let x = parser.next_f32()?;
        let y = parser.next_f32()?;
        Ok(Self { x, y })
    }
}

#[derive(Debug)]
pub struct Ratio {
    pub numerator: EncodedInt32,
    pub denominator: EncodedUint32,
}

impl Ratio {
    pub fn one() -> Self {
        Self::new(1, 1)
    }

    pub fn new(numerator: i32, denominator: u32) -> Self {
        Self {
            numerator: EncodedInt32::from(numerator),
            denominator: EncodedUint32::from(denominator),
        }
    }
}

impl AttributeValue for Ratio {}

impl Parsable for Ratio {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let numerator = parser.next_encoded_i32()?;
        let denominator = parser.next_encoded_u32()?;
        let result = Self {
            numerator,
            denominator,
        };
        log::debug!("parse_Ratio => {:?}", result);
        Ok(result)
    }
}

#[derive(Debug)]
pub struct AlphaStop {
    pub position: u16,
    pub midpoint: u16,
    pub opacity: u8,
}

impl Parsable for AlphaStop {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let position = parser.next_u16()?;
        let midpoint = parser.next_u16()?;
        let opacity = parser.next_u8()?;
        let result = Self {
            position,
            midpoint,
            opacity,
        };
        log::debug!("parselphaStop => {:?}", result);
        Ok(result)
    }
}

#[derive(Debug)]
pub struct ColorStop {
    pub position: u16,
    pub midpoint: u16,
    pub color: Color,
}

impl Parsable for ColorStop {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let position = parser.next_u16()?;
        let midpoint = parser.next_u16()?;
        let color = parser.next()?;
        let result = Self {
            position,
            midpoint,
            color,
        };
        log::debug!("parse_ColorStop => {:?}", result);
        Ok(result)
    }
}

#[derive(Debug)]
pub struct GradientColor {
    pub alpha_count: u32,
    pub color_count: u32,
    pub alpha_stop_list: Vec<AlphaStop>,
    pub color_stop_list: Vec<ColorStop>,
}

impl Parsable for GradientColor {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let alpha_count = parser.next_u32()?;
        let color_count = parser.next_u32()?;

        let mut alpha_stop_list = vec![];
        for _ in 0..alpha_count {
            let stop = AlphaStop::parse(parser, ctx.clone())?;
            alpha_stop_list.push(stop);
        }

        let mut color_stop_list = vec![];
        for _ in 0..color_count {
            let stop = ColorStop::parse(parser, ctx.clone())?;
            color_stop_list.push(stop);
        }

        let result = Self {
            alpha_count,
            color_count,
            alpha_stop_list,
            color_stop_list,
        };
        log::debug!("parse_GradientColor => {:?}", result);
        Ok(result)
    }
}

/// 混合模式
#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, ParsableEnum)]
#[repr(u8)]
pub enum BlendMode {
    Normal,
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// 轨道蒙版
#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, ParsableEnum)]
#[repr(u8)]
pub enum TrackMatteType {
    None,
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, ParsableEnum)]
#[repr(u8)]
pub enum LayerType {
    Null = 1,
    Solid = 2,
    Text = 3,
    Shape = 4,
    Image = 5,
    PreCompose = 6,
    Camera = 7,
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, ParsableEnum)]
#[repr(u8)]
pub enum ParagraphJustification {
    LeftJustify,
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// SolidColor 标识边框宽⾼以及颜⾊属性信息。
#[derive(Debug)]
pub struct SolidColor {
    pub solid_color: Color,
    pub width: EncodedInt32,
    pub height: EncodedInt32,
}

impl Parsable for SolidColor {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let solid_color = parser.next()?;
        let width = parser.next_encoded_i32()?;
        let height = parser.next_encoded_i32()?;
        let result = Self {
            solid_color,
            width,
            height,
        };
        log::debug!("parse_SolidColor => {:?}", result);
        Ok(result)
    }
}
