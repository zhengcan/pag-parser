use std::fmt::Debug;

use num_enum::FromPrimitive;
use num_enum::IntoPrimitive;

use crate::parse::Parsable;
use crate::parse::ParseContext;
use crate::parse::ParseError;
use crate::parse::Parser;
use crate::visit::LayerInfo;
use crate::visit::Traversable;

use super::image::*;
use super::layer::*;
use super::shape::*;
use super::text::*;
use super::video::*;
use super::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum TagCode {
    /// 结束标识
    End = 0,
    /// 字体集合，包含多个字体
    FontTables = 1,
    /// 矢量组合信息
    VectorCompositionBlock = 2,
    /// 组合基本属性信息
    CompositionAttributes = 3,
    /// 图片合集信息
    ImageTables = 4,
    /// 图层信息
    LayerBlock = 5,
    /// 图层基本属性信息
    LayerAttributes = 6,
    /// 边框颜色
    SolidColor = 7,
    /// 文本信息，包含：文本，字体，大小，颜色等基础信息
    TextSource = 8,
    /// 文本绘制信息，包含：绘制路径，前后左右间距等
    #[deprecated]
    DeprecatedTextPathOption = 9,
    /// 文本其他信息
    TextMoreOption = 10,
    /// 图片引用，指向一个图片
    ImageReference = 11,
    /// 组合引用，指向一个组合
    CompositionReference = 12,
    /// 2D 变换信息
    Transform2D = 13,
    /// 遮罩信息
    Mask = 14,
    /// Shape 信息
    ShapeGroup = 15,
    /// 矩形信息
    Rectangle = 16,
    /// 椭圆信息
    Ellipse = 17,
    /// 多边星形
    PolyStar = 18,
    /// Shape 路径信息
    ShapePath = 19,
    /// 填充规则信息
    Fill = 20,
    /// 描边
    Stroke = 21,
    /// 渐变填充
    GradientFill = 22,
    /// 渐变描边
    GradientStroke = 23,
    /// 合并路径
    MergePaths = 24,
    /// 裁剪路径
    TrimPaths = 25,
    /// 中继器
    Repeater = 26,
    /// 圆⻆
    RoundCorners = 27,
    /// 文件性能信息，主要用来校验 PAG 文件性能是否达标
    Performance = 28,
    /// 投影
    DropShadowStyle = 29,
    /// CachePolicy
    CachePolicy = 30,
    /// FileAttributes
    FileAttributes = 31,
    /// TimeStretchMode
    TimeStretchMode = 32,
    /// Mp4Header
    Mp4Header = 33,
    /// 位图序列帧
    BitmapCompositionBlock = 45,
    /// 位图序列
    BitmapSequence = 46,
    /// 图片字节流
    ImageBytes = 47,
    /// 图片字节流（带缩放）
    ImageBytes2 = 48,
    /// 图片字节流（带透明通道）
    ImageBytes3 = 49,
    /// 视频序列帧
    VideoCompositionBlock = 50,
    /// 视频序列
    VideoSequence = 51,
    /// LayerAttributesV2
    LayerAttributesV2 = 52,
    /// MarkerList
    MarkerList = 53,
    /// ImageFillRule
    ImageFillRule = 54,
    /// AudioBytes
    AudioBytes = 55,
    /// MotionTileEffect
    MotionTileEffect = 56,
    /// LevelsIndividualEffect
    LevelsIndividualEffect = 57,
    /// CornerPinEffect
    CornerPinEffect = 58,
    /// BulgeEffect
    BulgeEffect = 59,
    /// FastBlurEffect
    FastBlurEffect = 60,
    /// GlowEffect
    GlowEffect = 61,
    /// LayerAttributesV3
    LayerAttributesV3 = 62,
    /// LayerAttributesExtra
    LayerAttributesExtra = 63,
    /// TextSourceV2
    TextSourceV2 = 64,
    /// DropShadowStyleV2
    DropShadowStyleV2 = 65,
    /// DisplacementMapEffect
    DisplacementMapEffect = 66,
    /// ImageFillRuleV2
    ImageFillRuleV2 = 67,
    /// TextSourceV3
    TextSourceV3 = 68,
    /// TextPathOption
    TextPathOption = 69,
    /// TextAnimator
    TextAnimator = 70,
    /// TextRangeSelector
    TextRangeSelector = 71,
    /// TextAnimatorPropertiesTrackingType
    TextAnimatorPropertiesTrackingType = 72,
    /// TextAnimatorPropertiesTrackingAmount
    TextAnimatorPropertiesTrackingAmount = 73,
    /// TextAnimatorPropertiesFillColor
    TextAnimatorPropertiesFillColor = 74,
    /// TextAnimatorPropertiesStrokeColor
    TextAnimatorPropertiesStrokeColor = 75,
    /// TextAnimatorPropertiesPosition
    TextAnimatorPropertiesPosition = 76,
    /// TextAnimatorPropertiesScale
    TextAnimatorPropertiesScale = 77,
    /// TextAnimatorPropertiesRotation
    TextAnimatorPropertiesRotation = 78,
    /// TextAnimatorPropertiesOpacity
    TextAnimatorPropertiesOpacity = 79,
    /// TextWigglySelector
    TextWigglySelector = 80,
    /// RadialBlurEffect
    RadialBlurEffect = 81,
    /// MosaicEffect
    MosaicEffect = 82,
    /// EditableIndices
    EditableIndices = 83,
    /// MaskBlockV2
    MaskBlockV2 = 84,
    /// GradientOverlayStyle
    GradientOverlayStyle = 85,
    /// BrightnessContrastEffect
    BrightnessContrastEffect = 86,
    /// HueSaturationEffect
    HueSaturationEffect = 87,
    /// LayerAttributesExtraV2
    LayerAttributesExtraV2 = 88,
    /// EncryptedData
    EncryptedData = 89,
    /// Transform3D
    Transform3D = 90,
    /// CameraOption
    CameraOption = 91,
    /// StrokeStyle
    StrokeStyle = 92,
    /// OuterGlowStyle
    OuterGlowStyle = 93,
    /// ImageScaleModes
    ImageScaleModes = 94,
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Default)]
pub struct TagBlock {
    pub tags: Vec<Tag>,
}

impl TagBlock {
    pub fn push(&mut self, tag: Tag) {
        self.tags.push(tag);
    }

    pub fn find<P>(&self, predicate: P) -> Option<&Tag>
    where
        P: FnMut(&&Tag) -> bool,
    {
        self.tags.iter().find(predicate)
    }
}

impl Parsable for TagBlock {
    fn parse(parser: &mut impl Parser, ctx: impl ParseContext) -> Result<Self, ParseError> {
        let mut block = TagBlock { tags: vec![] };
        loop {
            let tag = Tag::parse(parser, ctx.clone())?;
            match tag.header.code {
                TagCode::End => {
                    return Ok(block);
                }
                _ => block.tags.push(tag),
            }
        }
    }
}

impl Traversable for TagBlock {
    fn traverse_layer<F>(&self, visitor: F)
    where
        F: Fn(&dyn LayerInfo) + Clone,
    {
        for tag in &self.tags {
            tag.traverse_layer(visitor.clone());
        }
    }
}

#[derive(Debug)]
pub struct Tag {
    pub header: TagHeader,
    pub body: TagBody,
}

impl Tag {
    pub const EMPTY: Tag = Self {
        header: TagHeader {
            code: TagCode::End,
            length: 0,
        },
        body: TagBody::End,
    };

    pub fn is_empty(&self) -> bool {
        self.header.code == TagCode::End
    }
}

impl Parsable for Tag {
    fn parse(parser: &mut impl Parser, ctx: impl ParseContext) -> Result<Self, ParseError> {
        log::debug!(
            "parse_Tag <= {} bytes: {:?}",
            parser.remain(),
            parser.peek(16)
        );

        let header = TagHeader::parse(parser, ctx.clone())?;
        let ctx = ctx.with_tag_code(header.code);
        let body = &mut parser.new_slice(header.length as usize)?;

        let body = match header.code {
            TagCode::End => TagBody::End,
            TagCode::FontTables => TagBody::FontTables(FontTables::parse(body, ctx)?),
            TagCode::VectorCompositionBlock => {
                TagBody::VectorCompositionBlock(VectorCompositionBlock::parse(body, ctx)?)
            }
            TagCode::CompositionAttributes => {
                TagBody::CompositionAttributes(CompositionAttributes::parse(body, ctx)?)
            }
            TagCode::ImageTables => TagBody::ImageTables(ImageTables::parse(body, ctx)?),
            TagCode::LayerBlock => TagBody::LayerBlock(LayerBlock::parse(body, ctx)?),
            TagCode::LayerAttributes => {
                TagBody::LayerAttributes(LayerAttributes::parse(body, ctx)?)
            }
            TagCode::SolidColor => TagBody::SolidColor(SolidColor::parse(body, ctx)?),
            TagCode::TextSource => TagBody::TextSource(TextSource::parse(body, ctx)?),
            TagCode::DeprecatedTextPathOption => {
                TagBody::TextPathOption(TextPathOption::parse(body, ctx)?)
            }
            TagCode::TextMoreOption => TagBody::TextMoreOption(TextMoreOption::parse(body, ctx)?),
            TagCode::ImageReference => TagBody::ImageReference(ImageReference::parse(body, ctx)?),
            TagCode::CompositionReference => {
                TagBody::CompositionReference(CompositionReference::parse(body, ctx)?)
            }
            TagCode::Transform2D => TagBody::Transform2D(Transform2D::parse(body, ctx)?),
            TagCode::Mask => TagBody::Mask(Mask::parse(body, ctx)?),
            // TagCode::ShapeGroup => TagBody::ShapeGroup(ShapeGroup::parse(body, ctx)?),
            // TagCode::Rectangle => TagBody::Rectangle(Rectangle::parse(body, ctx)?),
            // TagCode::Ellipse => TagBody::Ellipse(Ellipse::parse(body, ctx)?),
            // TagCode::PolyStar => TagBody::PolyStar(PolyStar::parse(body, ctx)?),
            // TagCode::ShapePath => TagBody::ShapePath(ShapePath::parse(body, ctx)?),
            // TagCode::Fill => TagBody::Fill(Fill::parse(body, ctx)?),
            // TagCode::Stroke => TagBody::Stroke(Stroke::parse(body, ctx)?),
            // TagCode::GradientFill => TagBody::GradientFill(GradientFill::parse(body, ctx)?),
            // TagCode::GradientStroke => TagBody::GradientStroke(GradientStroke::parse(body, ctx)?),
            // TagCode::MergePaths => TagBody::MergePaths(MergePaths::parse(body, ctx)?),
            // TagCode::TrimPaths => TagBody::TrimPaths(TrimPaths::parse(body, ctx)?),
            // TagCode::Repeater => TagBody::Repeater(Repeater::parse(body, ctx)?),
            // TagCode::RoundCorners => TagBody::RoundCorners(RoundCorners::parse(body, ctx)?),
            // TagCode::Performance => TagBody::Performance(Performance::parse(body, ctx)?),
            // TagCode::DropShadowStyle => {
            //     TagBody::DropShadowStyle(DropShadowStyle::parse(body, ctx)?)
            // }
            // TagCode::CachePolicy => TagBody::CachePolicy(CachePolicy::parse(body, ctx)?),
            TagCode::FileAttributes => TagBody::FileAttributes(FileAttributes::parse(body, ctx)?),
            // TagCode::TimeStretchMode => {
            //     TagBody::TimeStretchMode(TimeStretchMode::parse(body, ctx)?)
            // }
            // TagCode::Mp4Header => TagBody::Mp4Header(Mp4Header::parse(body, ctx)?),
            // TagCode::BitmapCompositionBlock => {
            //     TagBody::BitmapCompositionBlock(BitmapCompositionBlock::parse(body, ctx)?)
            // }
            // TagCode::BitmapSequence => TagBody::BitmapSequence(BitmapSequence::parse(body, ctx)?),
            TagCode::ImageBytes => TagBody::ImageBytes(ImageBytes::parse(body, ctx)?),
            TagCode::ImageBytes2 => TagBody::ImageBytes2(ImageBytes2::parse(body, ctx)?),
            TagCode::ImageBytes3 => TagBody::ImageBytes3(ImageBytes3::parse(body, ctx)?),
            TagCode::VideoCompositionBlock => {
                TagBody::VideoCompositionBlock(VideoCompositionBlock::parse(body, ctx)?)
            }
            TagCode::VideoSequence => TagBody::VideoSequence(VideoSequence::parse(body, ctx)?),
            TagCode::LayerAttributesV2 => {
                TagBody::LayerAttributesV2(LayerAttributes::parse(body, ctx)?)
            }
            // TagCode::MarkerList => TagBody::MarkerList(MarkerList::parse_block(body, ctx)?),
            // TagCode::ImageFillRule => TagBody::ImageFillRule(ImageFillRule::parse_block(body, ctx)?),
            // TagCode::AudioBytes => TagBody::AudioBytes(AudioBytes::parse_block(body, ctx)?),
            // TagCode::MotionTileEffect => {
            //     TagBody::MotionTileEffect(MotionTileEffect::parse_block(body, ctx)?)
            // }
            // TagCode::LevelsIndividualEffect => {
            //     TagBody::LevelsIndividualEffect(LevelsIndividualEffect::parse_block(body, ctx)?)
            // }
            // TagCode::CornerPinEffect => {
            //     TagBody::CornerPinEffect(CornerPinEffect::parse_block(body, ctx)?)
            // }
            // TagCode::BulgeEffect => TagBody::BulgeEffect(BulgeEffect::parse_block(body, ctx)?),
            // TagCode::FastBlurEffect => TagBody::FastBlurEffect(FastBlurEffect::parse_block(body, ctx)?),
            // TagCode::GlowEffect => TagBody::GlowEffect(GlowEffect::parse_block(body, ctx)?),
            TagCode::LayerAttributesV3 => {
                TagBody::LayerAttributesV3(LayerAttributes::parse(body, ctx)?)
            }
            TagCode::LayerAttributesExtra => {
                TagBody::LayerAttributesExtra(LayerAttributesExtra::parse(body, ctx)?)
            }
            // TagCode::TextSourceV2 => TagBody::TextSourceV2(TextSourceV2::parse_block(body, ctx)?),
            // TagCode::DropShadowStyleV2 => {
            //     TagBody::DropShadowStyleV2(DropShadowStyleV2::parse_block(body, ctx)?)
            // }
            // TagCode::DisplacementMapEffect => {
            //     TagBody::DisplacementMapEffect(DisplacementMapEffect::parse_block(body, ctx)?)
            // }
            // TagCode::ImageFillRuleV2 => {
            //     TagBody::ImageFillRuleV2(ImageFillRuleV2::parse_block(body, ctx)?)
            // }
            // TagCode::TextSourceV3 => TagBody::TextSourceV3(TextSourceV3::parse_block(body, ctx)?),
            TagCode::TextPathOption => TagBody::TextPathOption(TextPathOption::parse(body, ctx)?),
            // TagCode::TextAnimator => TagBody::TextAnimator(TextAnimator::parse_block(body, ctx)?),
            // TagCode::TextRangeSelector => {
            //     TagBody::TextRangeSelector(TextRangeSelector::parse_block(body, ctx)?)
            // }
            // TagCode::TextAnimatorPropertiesTrackingType => {
            //     TagBody::TextAnimatorPropertiesTrackingType(
            //         TextAnimatorPropertiesTrackingType::parse_block(body, ctx)?,
            //     )
            // }
            // TagCode::TextAnimatorPropertiesTrackingAmount => {
            //     TagBody::TextAnimatorPropertiesTrackingAmount(
            //         TextAnimatorPropertiesTrackingAmount::parse_block(body, ctx)?,
            //     )
            // }
            // TagCode::TextAnimatorPropertiesFillColor => TagBody::TextAnimatorPropertiesFillColor(
            //     TextAnimatorPropertiesFillColor::parse_block(body, ctx)?,
            // ),
            // TagCode::TextAnimatorPropertiesStrokeColor => {
            //     TagBody::TextAnimatorPropertiesStrokeColor(
            //         TextAnimatorPropertiesStrokeColor::parse_block(body, ctx)?,
            //     )
            // }
            // TagCode::TextAnimatorPropertiesPosition => TagBody::TextAnimatorPropertiesPosition(
            //     TextAnimatorPropertiesPosition::parse_block(body, ctx)?,
            // ),
            // TagCode::TextAnimatorPropertiesScale => TagBody::TextAnimatorPropertiesScale(
            //     TextAnimatorPropertiesScale::parse_block(body, ctx)?,
            // ),
            // TagCode::TextAnimatorPropertiesRotation => TagBody::TextAnimatorPropertiesRotation(
            //     TextAnimatorPropertiesRotation::parse_block(body, ctx)?,
            // ),
            // TagCode::TextAnimatorPropertiesOpacity => TagBody::TextAnimatorPropertiesOpacity(
            //     TextAnimatorPropertiesOpacity::parse_block(body, ctx)?,
            // ),
            // TagCode::TextWigglySelector => {
            //     TagBody::TextWigglySelector(TextWigglySelector::parse_block(body, ctx)?)
            // }
            // TagCode::RadialBlurEffect => {
            //     TagBody::RadialBlurEffect(RadialBlurEffect::parse_block(body, ctx)?)
            // }
            // TagCode::MosaicEffect => TagBody::MosaicEffect(MosaicEffect::parse_block(body, ctx)?),
            // TagCode::EditableIndices => {
            //     TagBody::EditableIndices(EditableIndices::parse_block(body, ctx)?)
            // }
            // TagCode::MaskBlockV2 => TagBody::MaskBlockV2(MaskBlockV2::parse_block(body, ctx)?),
            // TagCode::GradientOverlayStyle => {
            //     TagBody::GradientOverlayStyle(GradientOverlayStyle::parse_block(body, ctx)?)
            // }
            // TagCode::BrightnessContrastEffect => {
            //     TagBody::BrightnessContrastEffect(BrightnessContrastEffect::parse_block(body, ctx)?)
            // }
            // TagCode::HueSaturationEffect => {
            //     TagBody::HueSaturationEffect(HueSaturationEffect::parse_block(body, ctx)?)
            // }
            // TagCode::LayerAttributesExtraV2 => {
            //     TagBody::LayerAttributesExtraV2(LayerAttributesExtraV2::parse_block(body, ctx)?)
            // }
            // TagCode::EncryptedData => TagBody::EncryptedData(EncryptedData::parse_block(body, ctx)?),
            // TagCode::Transform3D => TagBody::Transform3D(Transform3D::parse_block(body, ctx)?),
            // TagCode::CameraOption => TagBody::CameraOption(CameraOption::parse_block(body, ctx)?),
            // TagCode::StrokeStyle => TagBody::StrokeStyle(StrokeStyle::parse_block(body, ctx)?),
            // TagCode::OuterGlowStyle => TagBody::OuterGlowStyle(OuterGlowStyle::parse_block(body, ctx)?),
            // TagCode::ImageScaleModes => {
            //     TagBody::ImageScaleModes(ImageScaleModes::parse_block(body, ctx)?)
            // }
            _ => TagBody::Raw(ByteData::from(body.buffer())),
        };
        Ok(Self { header, body })
    }
}

impl Traversable for Tag {
    fn traverse_layer<F>(&self, visitor: F)
    where
        F: Fn(&dyn LayerInfo) + Clone,
    {
        if let TagBody::LayerBlock(block) = &self.body {
            // log::info!("LayerBlock");
            block.traverse_layer(visitor);
        } else if let TagBody::VectorCompositionBlock(block) = &self.body {
            // log::info!("VectorCompositionBlock");
            block.traverse_layer(visitor);
        } else if let TagBody::VideoCompositionBlock(block) = &self.body {
            // log::info!("VideoCompositionBlock");
            block.traverse_layer(visitor);
        } else if let TagBody::BitmapCompositionBlock(block) = &self.body {
            // log::info!("BitmapCompositionBlock");
            block.traverse_layer(visitor);
        } else if let TagBody::ShapeGroup(block) = &self.body {
            // log::info!("ShapeGroup");
            block.traverse_layer(visitor);
        }
    }
}

#[derive(Debug)]
pub struct TagEnd {}

#[derive(Debug)]
pub struct TagHeader {
    pub code: TagCode,
    pub length: u32,
}

impl Parsable for TagHeader {
    fn parse(parser: &mut impl Parser, _ctx: impl ParseContext) -> Result<Self, ParseError> {
        const MASK: u32 = 0b0011_1111;

        let code_and_length = parser.next_u16()?;
        let code = (code_and_length >> 6) as u8;
        let mut length = code_and_length as u32 & MASK;
        if length == MASK {
            length = parser.next_u32()?;
        }
        let result = Self {
            code: TagCode::from(code),
            length,
        };

        log::debug!("parse_TagHeader => {:?}", result);
        Ok(result)
    }
}

#[derive(Debug)]
pub enum TagBody {
    /// 结束标识
    End,
    /// 字体集合，包含多个字体
    FontTables(FontTables),
    /// 矢量组合信息
    VectorCompositionBlock(VectorCompositionBlock),
    /// 组合基本属性信息
    CompositionAttributes(CompositionAttributes),
    /// 图片合集信息
    ImageTables(ImageTables),
    /// 图层信息
    LayerBlock(LayerBlock),
    /// 图层基本属性信息
    LayerAttributes(LayerAttributes),
    /// 边框颜色
    SolidColor(SolidColor),
    /// 文本信息，包含：文本，字体，大小，颜色等基础信息
    TextSource(TextSource),
    /// 文本绘制信息，包含：绘制路径，前后左右间距等
    TextPathOption(TextPathOption),
    /// 文本其他信息
    TextMoreOption(TextMoreOption),
    /// 图片引用，指向一个图片
    ImageReference(ImageReference),
    /// 组合引用，指向一个组合
    CompositionReference(CompositionReference),
    /// 2D 变换信息
    Transform2D(Transform2D),
    /// 遮罩信息
    Mask(Mask),
    /// Shape 信息
    ShapeGroup(ShapeGroup),
    /// 矩形信息
    Rectangle(Rectangle),
    /// 椭圆信息
    Ellipse(Ellipse),
    /// 多边星形
    PolyStar(PolyStar),
    /// Shape 路径信息
    ShapePath(ShapePath),
    /// 填充规则信息
    Fill(Fill),
    /// 描边
    Stroke(Stroke),
    /// 渐变填充
    GradientFill(GradientFill),
    /// 渐变描边
    GradientStroke(GradientStroke),
    /// 合并路径
    MergePaths(MergePaths),
    /// 裁剪路径
    TrimPaths(TrimPaths),
    /// 中继器
    Repeater(Repeater),
    /// 圆⻆
    RoundCorners(RoundCorners),
    /// 文件性能信息，主要用来校验 PAG 文件性能是否达标
    Performance(Performance),
    /// 投影
    DropShadowStyle(DropShadowStyle),
    /// 位图序列帧
    BitmapCompositionBlock(BitmapCompositionBlock),
    /// 位图序列
    BitmapSequence(BitmapSequence),
    /// 文件属性
    FileAttributes(FileAttributes),
    /// 图片字节流
    ImageBytes(ImageBytes),
    /// 图片字节流（带缩放）
    ImageBytes2(ImageBytes2),
    /// 图片字节流（带透明通道）
    ImageBytes3(ImageBytes3),
    /// 视频序列帧
    VideoCompositionBlock(VideoCompositionBlock),
    /// 视频序列
    VideoSequence(VideoSequence),
    /// 图层基本属性信息 V2
    LayerAttributesV2(LayerAttributes),
    /// 图层基本属性信息 V3
    LayerAttributesV3(LayerAttributes),
    /// 图层基本属性信息（额外）
    LayerAttributesExtra(LayerAttributesExtra),
    /// 未知
    Raw(ByteData),
}

/// Performance 标签主要存储 PAG 的性能指标数据。
#[derive(Debug)]
pub struct Performance {
    pub rendering_time: i64,
    pub image_decoding_time: i64,
    pub presenting_time: i64,
    pub graphics_memory: i64,
}
