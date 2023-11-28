use crate::format::*;

use super::{AttributeBlock, TagBlock};

/// VectorCompositionBlock 是⽮量图形的合集。⾥⾯可以包含简单的⽮量图形，也可以再包含⼀个或是多个 VectorComposition。
#[derive(Debug)]
pub struct VectorCompositionBlock {
    pub id: u32,
    pub tag_block: TagBlock,
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

/// ShapeGroup 投影标签。
#[derive(Debug)]
pub struct ShapeGroup {
    pub inner: AttributeBlock,
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
    pub inner: AttributeBlock,
    pub reversed: bool,
    pub size: Point,
    pub position: Point,
    pub roundness: f32,
}

/// Ellipse 标签。
#[derive(Debug)]
pub struct Ellipse {
    pub inner: AttributeBlock,
    pub reversed: bool,
    pub size: Point,
    pub position: Point,
}

/// 多边星形标签。
#[derive(Debug)]
pub struct PolyStar {
    pub inner: AttributeBlock,
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
    pub inner: AttributeBlock,
    pub shape_path: Path,
}

/// Fill 标签。
#[derive(Debug)]
pub struct Fill {
    pub inner: AttributeBlock,
    pub blend_mode: BlendMode,
    pub composite: CompositeOrder,
    pub fill_rule: FillRule,
    pub color: Color,
    pub opacity: u8,
}

/// Stroke 标签。
#[derive(Debug)]
pub struct Stroke {
    pub inner: AttributeBlock,
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
    pub inner: AttributeBlock,
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
    pub inner: AttributeBlock,
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
    pub inner: AttributeBlock,
    pub mode: MergePathsMode,
}

/// TrimPaths 标签。
#[derive(Debug)]
pub struct TrimPaths {
    pub inner: AttributeBlock,
    pub start: f32,
    pub end: f32,
    pub offset: f32,
    pub trim_type: TrimPathsType,
}

/// RoundCorners 标签。
#[derive(Debug)]
pub struct RoundCorners {
    pub inner: AttributeBlock,
    pub radius: f32,
}
