use nom::{
    number::complete::{le_f32, le_u32, le_u8},
    sequence::tuple,
    IResult,
};

use crate::format::{parse_bool, parse_enum, parse_time, AttributeType};

use super::{
    primitive::{parse_encode_u32, Time},
    AttributeBlock, AttributeConfig, BlendMode, Color, CompositeOrder, LayerType, MaskMode, Path,
    Point, Ratio, StreamParser, TagBlock, TrackMatteType,
};

/// LayerBlock 是图层信息的合集。
#[derive(Debug)]
pub struct LayerBlock {
    pub r#type: LayerType,
    pub id: u32,
    pub tag_block: TagBlock,
}

impl StreamParser for LayerBlock {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_LayerBlock <= {} bytes", input.len());
        let (input, (r#type, id)) = tuple((LayerType::parse, parse_encode_u32))(input)?;
        // log::warn!("{:?}", r#type);
        let (input, tag_block) = TagBlock::parse(input)?;
        let result = Self {
            r#type,
            id,
            tag_block,
        };
        log::debug!("parse_LayerBlock => {:?}", result);
        Ok((input, result))
    }
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

impl StreamParser for LayerAttributes {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_LayerAttributes <= {} bytes", input.len());

        let mut block = AttributeBlock::new(input);
        let is_active = block.flag(AttributeConfig::BitFlag(true));
        // v3 && !Camera => motionBlur
        let auto_orientation = block.flag(AttributeConfig::BitFlag(false));
        let parent = block.flag(AttributeConfig::Value(0));
        let stretch = block.flag(AttributeConfig::Value(Ratio::new(1, 1)));
        let start_time = block.flag(AttributeConfig::Value(0));
        // !Camera => blendMode
        let blend_mode = block.flag(AttributeConfig::Value(BlendMode::Normal));
        // !Camera => track_matte_type
        let track_matte_type = block.flag(AttributeConfig::Value(TrackMatteType::None));
        let time_remap = block.flag(AttributeConfig::SimpleProperty(0.));
        let duration = block.flag(AttributeConfig::FixedValue(0));
        // v2 || v3 => name

        let result = Self {
            is_active: block.read(is_active).unwrap_or(true),
            auto_orientation: block.read(auto_orientation).unwrap_or(false),
            parent: block.read(parent).unwrap_or(0),
            stretch: block.read(stretch).unwrap_or(Ratio::new(1, 1)),
            start_time: block.read(start_time).unwrap_or(0),
            blend_mode: block.read(blend_mode).unwrap_or(BlendMode::Normal),
            track_matte_type: block.read(track_matte_type).unwrap_or(TrackMatteType::None),
            time_remap: block.read(time_remap).unwrap_or(0.),
            duration: block.read(duration).unwrap_or(0),
        };

        let input = block.finish();
        log::debug!("parse_LayerAttributes => {:?}", result);
        Ok((input, result))
    }
}

/// CompositionReference 图层组合索引标签，存储的是⼀个图层组合的唯⼀ ID，通过 ID 索引真正的图层组合。
#[derive(Debug)]
pub struct CompositionReference {
    pub id: u32,
    pub composition_start_time: Time,
}

impl StreamParser for CompositionReference {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_CompositionReference <= {} bytes", input.len());
        let (input, (id, composition_start_time)) = tuple((parse_encode_u32, parse_time))(input)?;
        let result = Self {
            id,
            composition_start_time,
        };
        log::debug!("parse_CompositionReference => {:?}", result);
        Ok((input, result))
    }
}

/// Transform2D 2D 变换信息，包含：锚点，缩放，旋转，x 轴偏移，y 轴偏移等信息。
#[derive(Debug)]
pub struct Transform2D {
    pub anchor_point: Point,
    pub position: Point,
    pub x_position: f32,
    pub y_position: f32,
    pub scale: Point,
    pub rotation: f32,
    pub opacity: u8,
}

impl StreamParser for Transform2D {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_Transform2D <= {} bytes", input.len());

        let mut block = AttributeBlock::new(input);
        let anchor_point = block.flag(AttributeType::SpatialProperty);
        let position = block.flag(AttributeType::SpatialProperty);
        let x_position = block.flag(AttributeType::SimpleProperty);
        let y_position = block.flag(AttributeType::SimpleProperty);
        let scale = block.flag(AttributeType::MultiDimensionProperty);
        let rotation = block.flag(AttributeType::SimpleProperty);
        let opacity = block.flag(AttributeType::SimpleProperty);

        let result = Self {
            anchor_point: block.read(anchor_point).unwrap_or(Point::zero()),
            position: block.read(position).unwrap_or(Point::zero()),
            x_position: block.read(x_position).unwrap_or(0.),
            y_position: block.read(y_position).unwrap_or(0.),
            scale: block.read(scale).unwrap_or(Point::one()),
            rotation: block.read(rotation).unwrap_or(0.),
            opacity: block.read(opacity).unwrap_or(0xff),
        };
        let input = block.finish();

        log::debug!("parse_Transform2D => {:?}", result);
        Ok((input, result))
    }
}

/// Mask 遮罩标签。
#[derive(Debug)]
pub struct Mask {
    pub id: u32,
    pub inverted: bool,
    pub mask_mode: MaskMode,
    pub mask_path: Path,
    pub mask_opacity: u8,
    pub mask_expansion: f32,
}

impl StreamParser for Mask {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        log::debug!("parse_Mask <= {} bytes", input.len());
        let (input, (id, inverted, mask_mode, mask_path, mask_opacity, mask_expansion)) =
            tuple((le_u32, parse_bool, parse_enum, Path::parse, le_u8, le_f32))(input)?;
        let result = Self {
            id,
            inverted,
            mask_mode,
            mask_path,
            mask_opacity,
            mask_expansion,
        };
        log::debug!("parse_Mask => {:?}", result);
        Ok((input, result))
    }
}

/// Repeater 标签。
#[derive(Debug)]
pub struct Repeater {
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
    pub blend_mode: BlendMode,
    pub color: Color,
    pub opacity: u8,
    pub angle: f32,
    pub distance: f32,
    pub size: f32,
}
