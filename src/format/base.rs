use std::fmt::Debug;

use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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

#[derive(Debug)]
pub enum TrimPathsType {}

#[derive(Debug)]
pub enum MergePathsMode {}

#[derive(Debug)]
pub enum GradientFillType {}

#[derive(Debug)]
pub enum LineCap {}

#[derive(Debug)]
pub enum LineJoin {}

#[derive(Debug)]
pub enum CompositeOrder {}

#[derive(Debug)]
pub enum FillRule {}

#[derive(Debug)]
pub enum MaskMode {}

#[derive(Debug)]
pub struct Path {}

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Ratio {
    pub numerator: i32,
    pub denominator: u32,
}

#[derive(Debug)]
pub struct AlphaStop {
    pub position: u16,
    pub midpoint: u16,
    pub opacity: u8,
}

#[derive(Debug)]
pub struct ColorStop {
    pub position: u16,
    pub midpoint: u16,
    pub color: Color,
}

#[derive(Debug)]
pub struct GradientColor {
    pub alpha_count: u32,
    pub color_count: u32,
    pub alpha_stop_list: Vec<AlphaStop>,
    pub color_stop_list: Vec<ColorStop>,
}

pub type Time = u64;

/// 混合模式
#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum BlendMode {
    Normal,
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// 轨道蒙版
#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum TrackMatteType {
    None,
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum LayerType {
    /// 未知类型
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
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

// #[derive(Debug)]
// pub struct Bits<'a>(pub &'a [u8]);

// impl Bits<'_> {
//     pub fn get(&self, index: usize) -> bool {
//         let (i, j) = (index / 8, index % 8);
//         if i >= self.0.len() {
//             false
//         } else {
//             let byte = self.0[i];
//             (byte & (1 << j)) != 0
//         }
//     }

//     pub fn option<T>(&self, index: usize, value: T) -> Option<T> {
//         match self.get(index) {
//             true => Some(value),
//             false => None,
//         }
//     }
// }

#[derive(Debug)]
pub struct Bits<'a> {
    buffer: &'a [u8],
    count: usize,
    index: usize,
}

impl<'a> Bits<'a> {
    const OFFSET: usize = 3;

    pub fn new<'b>(buffer: &'b [u8]) -> Self
    where
        'b: 'a,
    {
        let count = (buffer[0] >> Self::OFFSET) as usize;
        Self {
            buffer,
            count,
            index: 0,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn bytes_count(&self) -> usize {
        ((8 - Self::OFFSET) + self.count + 7) / 8
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn has_next(&self) -> bool {
        self.index < self.count
    }

    pub fn next(&mut self) -> bool {
        if self.index >= self.count {
            return false;
        }

        let index = (8 - Self::OFFSET) + self.index;
        self.index += 1;
        self.get(index)
    }

    pub fn get(&self, index: usize) -> bool {
        if index >= self.count {
            return false;
        }

        let (i, j) = (index / 8, index % 8);
        if i >= self.buffer.len() {
            false
        } else {
            let byte = self.buffer[i];
            (byte & (1 << j)) != 0
        }
    }

    pub fn option<T>(&self, index: usize, value: T) -> Option<T> {
        match self.get(index) {
            true => Some(value),
            false => None,
        }
    }
}
