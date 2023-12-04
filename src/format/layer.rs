use crate::parse::{
    AttributeConfig, AttributeType, EncodedUint32, EncodedUint64, Parsable, ParseContext,
    ParseError, Parser, Time,
};

use super::{
    BlendMode, Color, CompositeOrder, LayerType, MaskMode, Path, Point, Ratio, TagBlock, TagCode,
    TrackMatteType,
};

/// LayerBlock 是图层信息的合集。
#[derive(Debug)]
pub struct LayerBlock {
    pub r#type: LayerType,
    pub id: EncodedUint32,
    pub tag_block: TagBlock,
}

impl Parsable for LayerBlock {
    fn parse(parser: &mut impl Parser, ctx: impl ParseContext) -> Result<Self, ParseError> {
        let r#type = parser.next_enum()?;
        let id = parser.next_id()?;
        let tag_block = TagBlock::parse(parser, ctx.with_layer_type(r#type))?;
        let result = Self {
            r#type,
            id,
            tag_block,
        };
        log::debug!("parse_LayerBlock => {:?}", result);
        Ok(result)
    }
}

/// LayerAttributes 是图层的属性信息。
#[derive(Debug)]
pub struct LayerAttributes {
    pub is_active: bool,
    pub auto_orientation: bool,
    pub motion_blur: bool,
    pub parent: EncodedUint32,
    pub stretch: Ratio,
    pub start_time: Time,
    pub blend_mode: BlendMode,
    pub track_matte_type: TrackMatteType,
    pub time_remap: f32,
    pub duration: Time,
    pub name: String,
}

impl Parsable for LayerAttributes {
    fn parse(parser: &mut impl Parser, ctx: impl ParseContext) -> Result<Self, ParseError> {
        let mut block = parser.new_attribute_block();
        let is_active = block.flag(AttributeConfig::BitFlag(true));
        let auto_orientation = block.flag(AttributeConfig::BitFlag(false));
        // v3 && !Camera => motionBlur
        let motion_blur = match (ctx.parent_code(), ctx.layer_type()) {
            (Some(TagCode::LayerAttributesV3), Some(LayerType::Camera)) => {
                block.flag(AttributeType::NotExisted)
            }
            (Some(TagCode::LayerAttributesV3), Some(_)) => {
                block.flag(AttributeConfig::BitFlag(false))
            }
            _ => block.flag(AttributeType::NotExisted),
        };
        let parent = block.flag(AttributeConfig::Value(0));
        let stretch = block.flag(AttributeConfig::Value(Ratio::new(1, 1)));
        let start_time = block.flag(AttributeConfig::Value(0));
        // !Camera => blendMode
        let blend_mode = match ctx.layer_type() {
            Some(LayerType::Camera) => block.flag(AttributeType::NotExisted),
            _ => block.flag(AttributeConfig::Value(BlendMode::Normal)),
        };
        // !Camera => track_matte_type
        let track_matte_type = match ctx.layer_type() {
            Some(LayerType::Camera) => block.flag(AttributeType::NotExisted),
            _ => block.flag(AttributeConfig::Value(TrackMatteType::None)),
        };
        let time_remap = block.flag(AttributeConfig::SimpleProperty(0.));
        let duration = block.flag(AttributeConfig::FixedValue(0));
        // v2 || v3 => name
        let name = match ctx.parent_code() {
            Some(TagCode::LayerAttributesV2) | Some(TagCode::LayerAttributesV3) => {
                block.flag(AttributeConfig::Value("".to_string()))
            }
            _ => block.flag(AttributeType::NotExisted),
        };

        let mut result = Self {
            is_active: block.read(is_active).unwrap_or(true),
            auto_orientation: block.read(auto_orientation).unwrap_or(false),
            motion_blur: block.read(motion_blur).unwrap_or(false),
            parent: block.read(parent).unwrap_or(EncodedUint32::from(0)),
            stretch: block.read(stretch).unwrap_or(Ratio::one()),
            start_time: block.read(start_time).unwrap_or(EncodedUint64::from(0)),
            blend_mode: block.read(blend_mode).unwrap_or(BlendMode::Normal),
            track_matte_type: block.read(track_matte_type).unwrap_or(TrackMatteType::None),
            time_remap: block.read(time_remap).unwrap_or(0.),
            duration: block.read(duration).unwrap_or(EncodedUint64::from(0)),
            name: block.read(name).unwrap_or_default(),
        };

        // The duration can not be zero, fix it when the value is parsed from an old file format.
        if result.duration == 0 {
            result.duration = Time::from(1);
        }

        // let input = block.finish();
        log::debug!("parse_LayerAttributes => {:?}", result);
        Ok(result)
    }
}

#[derive(Debug)]
pub struct LayerAttributesExtra {
    pub name: String,
    pub motion_blur: bool,
}

impl Parsable for LayerAttributesExtra {
    fn parse(parser: &mut impl Parser, ctx: impl ParseContext) -> Result<Self, ParseError> {
        let mut block = parser.new_attribute_block();
        let name = block.flag(AttributeConfig::Value("".to_string()));
        // !Camera => motionBlur
        let motion_blur = match ctx.layer_type() {
            Some(LayerType::Camera) => block.flag(AttributeType::NotExisted),
            Some(_) => block.flag(AttributeConfig::BitFlag(false)),
            _ => block.flag(AttributeType::NotExisted),
        };

        let result = Self {
            name: block.read(name).unwrap_or_default(),
            motion_blur: block.read(motion_blur).unwrap_or(false),
        };

        // let input = block.finish();
        log::debug!("parse_LayerAttributesExtra => {:?}", result);
        Ok(result)
    }
}

/// CompositionReference 图层组合索引标签，存储的是⼀个图层组合的唯⼀ ID，通过 ID 索引真正的图层组合。
#[derive(Debug)]
pub struct CompositionReference {
    pub id: EncodedUint32,
    pub composition_start_time: Time,
}

impl Parsable for CompositionReference {
    fn parse(parser: &mut impl Parser, _ctx: impl ParseContext) -> Result<Self, ParseError> {
        let id = parser.next_encoded_u32()?;
        let composition_start_time = parser.next_time()?;
        let result = Self {
            id,
            composition_start_time,
        };
        log::debug!("parse_CompositionReference => {:?}", result);
        Ok(result)
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

impl Parsable for Transform2D {
    fn parse(parser: &mut impl Parser, _ctx: impl ParseContext) -> Result<Self, ParseError> {
        let mut block = parser.new_attribute_block();
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
        // let input = block.finish();

        log::debug!("parse_Transform2D => {:?}", result);
        Ok(result)
    }
}

/// Mask 遮罩标签。
#[derive(Debug)]
pub struct Mask {
    pub id: EncodedUint32,
    pub inverted: bool,
    pub mask_mode: MaskMode,
    pub mask_path: Path,
    pub mask_opacity: u8,
    pub mask_expansion: f32,
}

impl Parsable for Mask {
    fn parse(parser: &mut impl Parser, ctx: impl ParseContext) -> Result<Self, ParseError> {
        let id = parser.next_id()?;
        let inverted = parser.next_bool()?;
        let mask_mode = parser.next_enum()?;
        let mask_path = Path::parse(parser, ctx)?;
        let mask_opacity = parser.next_u8()?;
        let mask_expansion = parser.next_f32()?;
        let result = Self {
            id,
            inverted,
            mask_mode,
            mask_path,
            mask_opacity,
            mask_expansion,
        };
        log::debug!("parse_Mask => {:?}", result);
        Ok(result)
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
