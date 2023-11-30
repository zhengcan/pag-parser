use std::fmt::Debug;

use macros::Parsable;
use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_u16, le_u32, le_u8},
    sequence::tuple,
    IResult,
};
use num_enum::{FromPrimitive, IntoPrimitive};

use super::{
    primitive::{parse_encode_i32, parse_encode_u32},
    StreamParser,
};

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl StreamParser for Color {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_Color <= {} bytes", input.len());
        let (input, (red, green, blue)) = tuple((le_u8, le_u8, le_u8))(input)?;
        let result = Self { red, green, blue };
        log::debug!("parse_Color => {:?}", result);
        Ok((input, result))
    }
}

pub struct ByteData {
    pub length: u32,
    pub data: Vec<u8>,
}

impl ByteData {
    pub fn from(data: &[u8]) -> Self {
        Self {
            length: data.len() as u32,
            data: Vec::from(data),
        }
    }
}

impl Debug for ByteData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ByteData").field(&self.length).finish()
    }
}

impl StreamParser for ByteData {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_ByteData <= {} bytes", input.len());
        let (input, length) = parse_encode_u32(input)?;
        assert!(
            length <= input.len() as u32,
            "EOF: expect={}, actual={}",
            length,
            input.len()
        );
        let (input, data) = take(length)(input)?;
        assert_eq!(length, data.len() as u32);
        let result = Self {
            length,
            data: Vec::from(data),
        };
        log::debug!("parse_ByteData => {:?}", result);
        Ok((input, result))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, Parsable)]
#[repr(u8)]
pub enum TrimPathsType {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, Parsable)]
#[repr(u8)]
pub enum MergePathsMode {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, Parsable)]
#[repr(u8)]
pub enum GradientFillType {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, Parsable)]
#[repr(u8)]
pub enum LineCap {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, Parsable)]
#[repr(u8)]
pub enum LineJoin {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, Parsable)]
#[repr(u8)]
pub enum CompositeOrder {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, Parsable)]
#[repr(u8)]
pub enum FillRule {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, Parsable)]
#[repr(u8)]
pub enum MaskMode {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug)]
pub struct Path {}

impl StreamParser for Path {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_Path <= {} bytes", input.len());
        let result = Self {};
        log::debug!("parse_Path => {:?}", result);
        Ok((input, result))
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

impl StreamParser for Point {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        // log::debug!("parse_Point <= {} bytes", input.len());
        let (input, (x, y)) = tuple((le_f32, le_f32))(input)?;
        let result = Self { x, y };
        log::debug!("parse_Point => {:?}", result);
        Ok((input, result))
    }
}

#[derive(Debug)]
pub struct Ratio {
    pub numerator: i32,
    pub denominator: u32,
}

impl Ratio {
    pub fn new(numerator: i32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl StreamParser for Ratio {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_Ratio <= {} bytes", input.len());
        let (input, (numerator, denominator)) = tuple((parse_encode_i32, parse_encode_u32))(input)?;
        let result = Self {
            numerator,
            denominator,
        };
        log::debug!("parse_Ratio => {:?}", result);
        Ok((input, result))
    }
}

#[derive(Debug)]
pub struct AlphaStop {
    pub position: u16,
    pub midpoint: u16,
    pub opacity: u8,
}

impl StreamParser for AlphaStop {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_AlphaStop <= {} bytes", input.len());
        let (input, (position, midpoint, opacity)) = tuple((le_u16, le_u16, le_u8))(input)?;
        let result = Self {
            position,
            midpoint,
            opacity,
        };
        log::debug!("parse_AlphaStop => {:?}", result);
        Ok((input, result))
    }
}

#[derive(Debug)]
pub struct ColorStop {
    pub position: u16,
    pub midpoint: u16,
    pub color: Color,
}

impl StreamParser for ColorStop {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_ColorStop <= {} bytes", input.len());
        let (input, (position, midpoint, color)) = tuple((le_u16, le_u16, Color::parse))(input)?;
        let result = Self {
            position,
            midpoint,
            color,
        };
        log::debug!("parse_ColorStop => {:?}", result);
        Ok((input, result))
    }
}

#[derive(Debug)]
pub struct GradientColor {
    pub alpha_count: u32,
    pub color_count: u32,
    pub alpha_stop_list: Vec<AlphaStop>,
    pub color_stop_list: Vec<ColorStop>,
}

impl StreamParser for GradientColor {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_GradientColor <= {} bytes", input.len());
        let (mut input, (alpha_count, color_count)) = tuple((le_u32, le_u32))(input)?;

        let mut alpha_stop_list = vec![];
        for _ in 0..alpha_count {
            let (next, stop) = AlphaStop::parse(input)?;
            input = next;
            alpha_stop_list.push(stop);
        }

        let mut color_stop_list = vec![];
        for _ in 0..color_count {
            let (next, stop) = ColorStop::parse(input)?;
            input = next;
            color_stop_list.push(stop);
        }

        let result = Self {
            alpha_count,
            color_count,
            alpha_stop_list,
            color_stop_list,
        };
        log::debug!("parse_GradientColor => {:?}", result);
        Ok((input, result))
    }
}

/// 混合模式
#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, Parsable)]
#[repr(u8)]
pub enum BlendMode {
    Normal,
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// 轨道蒙版
#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, Parsable)]
#[repr(u8)]
pub enum TrackMatteType {
    None,
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, Parsable)]
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, Parsable)]
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
    pub width: i32,
    pub height: i32,
}

impl StreamParser for SolidColor {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_SolidColor <= {} bytes", input.len());
        let (input, (solid_color, width, height)) =
            tuple((Color::parse, parse_encode_i32, parse_encode_i32))(input)?;
        let result = Self {
            solid_color,
            width,
            height,
        };
        log::debug!("parse_SolidColor => {:?}", result);
        Ok((input, result))
    }
}
