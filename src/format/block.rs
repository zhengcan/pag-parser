use std::fmt::Debug;

use nom::IResult;
use num_enum::FromPrimitive;
use num_enum::IntoPrimitive;

use super::image::*;
use super::layer::*;
use super::shape::*;
use super::text::*;
use super::video::*;
use super::*;

#[derive(Debug)]
pub struct TagBlock {
    pub tags: Vec<Tag>,
}

#[derive(Debug)]
pub struct Tag {
    pub header: TagHeader,
    pub body: TagBody,
}

#[derive(Debug)]
pub struct TagEnd {}

#[derive(Debug)]
pub struct TagHeader {
    pub code: TagCode,
    pub length: u32,
}

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
    TextPathOption = 9,
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
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
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
    /// 未知
    Raw(ByteData),
}

#[derive(Debug)]
pub struct AttributeBlock {
    pub flags: Vec<AttributeFlag>,
    pub buffer: Vec<u8>,
    pub contents: Vec<AttributeContent>,
}

// impl AttributeBlock {
//     pub fn get<T>(&self, index: usize) -> Option<T> {
//         // Skip previous attributes
//         let mut buf = self.buffer.as_slice();
//         for i in 0..index {
//             match self.flags.get(i) {
//                 Some(flag) => {},
//                 None => {},
//             };
//         }
//         // Retrieve attribute

//         self.flags.get(index).map(|flag| match flag {
//             AttributeFlag::NotExisted => None,
//             _ => {
//                 Some(T::default())
//             }
//         })
//     }
// }

#[derive(Debug)]
pub enum AttributeFlag {
    Existed,
    ExistedAnimatable,
    ExistedAnimatableSpatial,
    NotExisted,
}

#[derive(Debug)]
pub struct AttributeContent {}

/// Performance 标签主要存储 PAG 的性能指标数据。
#[derive(Debug)]
pub struct Performance {
    pub rendering_time: i64,
    pub image_decoding_time: i64,
    pub presenting_time: i64,
    pub graphics_memory: i64,
}
