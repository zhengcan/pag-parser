use crate::parse::{
    AttributeConfig, AttributeValue, EncodedUint32, Parsable, ParseError, Parser, ParserContext,
};

use super::{Color, ParagraphJustification, Point};

/// FontTables 是字体信息的合集。
#[derive(Debug)]
pub struct FontTables {
    pub count: EncodedUint32,
    pub font_datas: Vec<FontData>,
}

impl Parsable for FontTables {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let count = parser.next_encoded_u32()?;
        let mut font_datas = vec![];
        for _ in 0..count.to_u32() {
            let font_data = FontData::parse(parser, ctx.clone())?;
            font_datas.push(font_data);
        }
        let result = Self { count, font_datas };
        log::debug!("parse_FontTables => {:?}", result);
        Ok(result)
    }
}

/// FontData 标识字体
#[derive(Debug)]
pub struct FontData {
    pub font_family: String,
    pub font_style: String,
}

impl Parsable for FontData {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let font_family = parser.next_string()?;
        let font_style = parser.next_string()?;
        let result = Self {
            font_family,
            font_style,
        };
        log::debug!("parse_FontData => {:?}", result);
        Ok(result)
    }
}

#[derive(Debug, Default)]
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
    pub font_id: Option<EncodedUint32>,
}

impl TextDocument {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AttributeValue for TextDocument {}

impl Parsable for TextDocument {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let mut bits = parser.new_bits();
        // log::debug!("bits = {:?}", bits);

        let apply_fill_flag = bits.next();
        let apply_stroke_flag = bits.next();
        let box_text_flag = bits.next();
        let faux_bold_flag = bits.next();
        let faux_italic_flag = bits.next();
        let stroke_over_fill_flag = bits.next();
        let baseline_shift_flag = bits.next();
        let first_baseline_flag = bits.next();
        let box_text_pos_flag = bits.next();
        let box_text_size_flag = bits.next();
        let fill_color_flag = bits.next();
        let font_size_flag = bits.next();
        let stroke_color_flag = bits.next();
        let stroke_width_flag = bits.next();
        let text_flag = bits.next();
        let justification_flag = bits.next();
        let leading_flag = bits.next();
        let tracking_flag = bits.next();
        let has_font_data_flag = bits.next();
        let parser = &mut bits.finish();

        let mut result = Self {
            apply_fill_flag,
            apply_stroke_flag,
            box_text_flag,
            faux_bold_flag,
            faux_italic_flag,
            stroke_over_fill_flag,
            baseline_shift_flag,
            first_baseline_flag,
            box_text_pos_flag,
            box_text_size_flag,
            fill_color_flag,
            font_size_flag,
            stroke_color_flag,
            stroke_width_flag,
            text_flag,
            justification_flag,
            leading_flag,
            tracking_flag,
            has_font_data_flag,
            ..Default::default()
        };

        if baseline_shift_flag {
            result.baseline_shift = Some(parser.next_f32()?);
        }
        if first_baseline_flag {
            result.first_baseline = Some(parser.next_f32()?);
        }
        if box_text_pos_flag {
            result.box_text_pos = Some(parser.next()?);
        }
        if box_text_size_flag {
            result.box_text_size = Some(parser.next()?);
        }
        if fill_color_flag {
            result.fill_color = Some(parser.next()?);
        }
        if font_size_flag {
            result.font_size = Some(parser.next_f32()?);
        }
        if stroke_color_flag {
            result.stroke_color = Some(parser.next()?);
        }
        if stroke_width_flag {
            result.stroke_width = Some(parser.next_f32()?);
        }
        if text_flag {
            result.text = Some(parser.next_string()?);
        }
        if justification_flag {
            result.justification = Some(parser.next_u8()?);
        }
        if leading_flag {
            result.leading = Some(parser.next_f32()?);
        }
        if tracking_flag {
            result.tracking = Some(parser.next_f32()?);
        }
        if has_font_data_flag {
            result.font_id = Some(parser.next_encoded_u32()?);
        }

        log::debug!("parse_TextDocument => {:?}", result);
        Ok(result)
    }
}

/// TextPathOption ⽂本绘制信息，包含：绘制路径，前后左右间距等。
#[derive(Debug)]
pub struct TextPathOption {
    // pub inner: AttributeBlock,
    pub path: EncodedUint32,
    pub reversed_path: bool,
    pub perpendicular_to_path: bool,
    pub force_alignment: bool,
    pub first_margin: f32,
    pub last_margin: f32,
}

impl Parsable for TextPathOption {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let mut block = parser.new_attribute_block();
        let path = block.flag(AttributeConfig::Value(0)); // EncodedUint32
        let reversed_path = block.flag(AttributeConfig::DiscreteProperty(false));
        let perpendicular_to_path = block.flag(AttributeConfig::DiscreteProperty(false));
        let force_aligment = block.flag(AttributeConfig::DiscreteProperty(false));
        let first_margin = block.flag(AttributeConfig::SimpleProperty(0.));
        let last_margin = block.flag(AttributeConfig::SimpleProperty(0.));

        let result = Self {
            path: block.read(path).unwrap_or(EncodedUint32::from(0)),
            reversed_path: block.read(reversed_path).unwrap_or(false),
            perpendicular_to_path: block.read(perpendicular_to_path).unwrap_or(false),
            force_alignment: block.read(force_aligment).unwrap_or(false),
            first_margin: block.read(first_margin).unwrap_or(0.),
            last_margin: block.read(last_margin).unwrap_or(0.),
        };
        // let input = block.finish();

        log::debug!("parse_TextPathOption => {:?}", result);
        Ok(result)
    }
}

#[derive(Debug)]
pub struct TextMoreOption {
    // pub inner: AttributeBlock,
    pub anchor_point_grouping: ParagraphJustification,
    pub grouping_alignment: Point,
}

impl Parsable for TextMoreOption {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let mut block = parser.new_attribute_block();
        let anchor_point_grouping =
            block.flag(AttributeConfig::Value(ParagraphJustification::LeftJustify));
        let grouping_alignment = block.flag(AttributeConfig::MultiDimensionProperty(Point::zero()));

        let result = Self {
            anchor_point_grouping: block
                .read(anchor_point_grouping)
                .unwrap_or(ParagraphJustification::LeftJustify),
            grouping_alignment: block.read(grouping_alignment).unwrap_or(Point::zero()),
        };
        // let input = block.finish();

        log::debug!("parse_TextMoreOption => {:?}", result);
        Ok(result)
    }
}

/// TextSource ⽂本信息，包含：⽂本，字体，⼤⼩，颜⾊等基础信息。
#[derive(Debug)]
pub struct TextSource {
    // pub inner: AttributeBlock,
    pub source_text: TextDocument,
}

impl Parsable for TextSource {
    fn parse(parser: &mut impl Parser, ctx: impl ParserContext) -> Result<Self, ParseError> {
        let mut block = parser.new_attribute_block();
        let source_text = block.flag(AttributeConfig::DiscreteProperty(TextDocument::new())); // ??

        let result = Self {
            source_text: block.read(source_text).unwrap_or(TextDocument::new()), // ??
        };

        log::debug!("parse_TextSource => {:?}", result);
        Ok(result)
    }
}
