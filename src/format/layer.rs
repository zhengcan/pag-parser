use super::*;

/// LayerBlock 是图层信息的合集。
#[derive(Debug)]
pub struct LayerBlock {
    pub r#type: LayerType,
    pub id: u32,
    pub tag_block: TagBlock,
}

/// LayerAttributes 是图层的属性信息。
#[derive(Debug)]
pub struct LayerAttributes {
    // pub inner: AttributeBlock,
    pub is_active: bool,
    pub auto_orientation: bool,
    pub parent: u32,
    pub stretch: Ratio,
    pub start_time: Time,
    pub blend_mode: BlendMode,
    pub track_matte_type: TrackMatteType,
    pub time_remap: f32,
    pub duration: Time,
}

/// CompositionReference 图层组合索引标签，存储的是⼀个图层组合的唯⼀ ID，通过 ID 索引真正的图层组合。
#[derive(Debug)]
pub struct CompositionReference {
    pub inner: AttributeBlock,
    pub id: u32,
    pub composition_start_time: Time,
}

/// Transform2D 2D 变换信息，包含：锚点，缩放，旋转，x 轴偏移，y 轴偏移等信息。
#[derive(Debug)]
pub struct Transform2D {
    pub inner: AttributeBlock,
    pub anchor_point: Point,
    pub position: Point,
    pub x_position: f32,
    pub y_position: f32,
    pub scale: Point,
    pub rotation: f32,
    pub opacity: u8,
}

/// Mask 遮罩标签。
#[derive(Debug)]
pub struct Mask {
    pub inner: AttributeBlock,
    pub id: u32,
    pub inverted: bool,
    pub mask_mode: MaskMode,
    pub mask_path: Path,
    pub mask_opacity: u8,
    pub mask_expansion: f32,
}

/// Repeater 标签。
#[derive(Debug)]
pub struct Repeater {
    pub inner: AttributeBlock,
    pub composite: CompositeOrder,
    pub copies: f32,
    pub offset: f32,
    pub anchor_point: Point,
    pub position: Point,
    pub scale: Point,
    pub rotation: f32,
    pub start_opacity: u8,
    pub end_opacity: u8,
}

/// DropShadowStyle 标签。
#[derive(Debug)]
pub struct DropShadowStyle {
    pub inner: AttributeBlock,
    pub blend_mode: BlendMode,
    pub color: Color,
    pub opacity: u8,
    pub angle: f32,
    pub distance: f32,
    pub size: f32,
}
