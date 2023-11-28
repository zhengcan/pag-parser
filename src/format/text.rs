use crate::format::Point;

use super::{AttributeBlock, Color, ParagraphJustification};

/// FontTables 是字体信息的合集。
#[derive(Debug)]
pub struct FontTables {
    pub count: u32,
    pub font_datas: Vec<FontData>,
}

/// FontData 标识字体
#[derive(Debug)]
pub struct FontData {
    pub font_family: String,
    pub font_style: String,
}

#[derive(Debug)]
pub struct TextDocument {
    pub apply_fill_flag: bool,
    pub apply_stroke_flag: bool,
    pub box_text_flag: bool,
    pub faux_bold_flag: bool,
    pub faux_italic_flag: bool,
    pub stroke_over_fill_flag: bool,
    pub baseline_shift_flag: bool,
    pub first_baseline_flag: bool,
    pub box_text_pos_flag: bool,
    pub box_text_size_flag: bool,
    pub fill_color_flag: bool,
    pub font_size_flag: bool,
    pub stroke_color_flag: bool,
    pub stroke_width_flag: bool,
    pub text_flag: bool,
    pub justification_flag: bool,
    pub leading_flag: bool,
    pub tracking_flag: bool,
    pub has_font_data_flag: bool,
    //--
    pub baseline_shift: Option<f32>,
    pub first_baseline: Option<f32>,
    pub box_text_pos: Option<Point>,
    pub box_text_size: Option<Point>,
    pub fill_color: Option<Color>,
    pub font_size: Option<f32>,
    pub stroke_color: Option<Color>,
    pub stroke_width: Option<f32>,
    pub text: Option<String>,
    pub justification: Option<u8>,
    pub leading: Option<f32>,
    pub tracking: Option<f32>,
    pub font_id: Option<u32>,
}

/// TextPathOption ⽂本绘制信息，包含：绘制路径，前后左右间距等。
#[derive(Debug)]
pub struct TextPathOption {
    // pub inner: AttributeBlock,
    pub path: u32,
    pub reversed_path: bool,
    pub perpendicular_to_path: bool,
    pub force_alignment: bool,
    pub first_margin: f32,
    pub last_margin: f32,
}

#[derive(Debug)]
pub struct TextMoreOption {
    // pub inner: AttributeBlock,
    pub anchor_point_grouping: ParagraphJustification,
    pub grouping_alignment: Point,
}

/// TextSource ⽂本信息，包含：⽂本，字体，⼤⼩，颜⾊等基础信息。
#[derive(Debug)]
pub struct TextSource {
    // pub inner: AttributeBlock,
    pub source_text: TextDocument,
}
