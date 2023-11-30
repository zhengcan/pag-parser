use nom::{
    number::complete::{le_f32, le_u8},
    sequence::tuple,
};

use crate::format::{
    primitive::{parse_encode_u32, parse_string},
    AttributeConfig, Bits, Point,
};

use super::{AttributeBlock, Color, ParagraphJustification, StreamParser};

/// FontTables 是字体信息的合集。
#[derive(Debug)]
pub struct FontTables {
    pub count: u32,
    pub font_datas: Vec<FontData>,
}

impl StreamParser for FontTables {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        log::debug!("parse_FontTables <= {} bytes", input.len());
        let (mut input, count) = parse_encode_u32(input)?;
        let mut font_datas = vec![];
        for _ in 0..count {
            let (next, font_data) = FontData::parse(input)?;
            input = next;
            font_datas.push(font_data);
        }
        let result = Self { count, font_datas };
        log::debug!("parse_FontTables => {:?}", result);
        Ok((input, result))
    }
}

/// FontData 标识字体
#[derive(Debug)]
pub struct FontData {
    pub font_family: String,
    pub font_style: String,
}

impl StreamParser for FontData {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        log::debug!("parse_FontData <= {} bytes", input.len());
        let (input, (font_family, font_style)) = tuple((parse_string, parse_string))(input)?;
        let result = Self {
            font_family,
            font_style,
        };
        log::debug!("parse_FontData => {:?}", result);
        Ok((input, result))
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
    pub font_id: Option<u32>,
}

impl TextDocument {
    pub fn new() -> Self {
        Self::default()
    }
}

impl StreamParser for TextDocument {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        log::debug!("parse_TextDocument <= {} bytes: {:?}", input.len(), input);
        let mut bits = Bits::new(input);
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
        let mut input = bits.finish();

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
            let (next, value) = le_f32(input)?;
            result.baseline_shift = Some(value);
            input = next;
        }
        if first_baseline_flag {
            let (next, value) = le_f32(input)?;
            result.first_baseline = Some(value);
            input = next;
        }
        if box_text_pos_flag {
            let (next, value) = Point::parse(input)?;
            result.box_text_pos = Some(value);
            input = next;
        }
        if box_text_size_flag {
            let (next, value) = Point::parse(input)?;
            result.box_text_size = Some(value);
            input = next;
        }
        if fill_color_flag {
            let (next, value) = Color::parse(input)?;
            result.fill_color = Some(value);
            input = next;
        }
        if font_size_flag {
            let (next, value) = le_f32(input)?;
            result.font_size = Some(value);
            input = next;
        }
        if stroke_color_flag {
            let (next, value) = Color::parse(input)?;
            result.stroke_color = Some(value);
            input = next;
        }
        if stroke_width_flag {
            let (next, value) = le_f32(input)?;
            result.stroke_width = Some(value);
            input = next;
        }
        if text_flag {
            let (next, value) = parse_string(input)?;
            result.text = Some(value);
            input = next;
        }
        if justification_flag {
            let (next, value) = le_u8(input)?;
            result.justification = Some(value);
            input = next;
        }
        if leading_flag {
            let (next, value) = le_f32(input)?;
            result.leading = Some(value);
            input = next;
        }
        if tracking_flag {
            let (next, value) = le_f32(input)?;
            result.tracking = Some(value);
            input = next;
        }
        if has_font_data_flag {
            let (next, value) = parse_encode_u32(input)?;
            result.font_id = Some(value);
            input = next;
        }

        log::debug!("parse_TextDocument => {:?}", result);
        Ok((input, result))
    }
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

impl StreamParser for TextPathOption {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        log::debug!("parse_TextPathOption <= {} bytes", input.len());

        let mut block = AttributeBlock::new(input);
        let path = block.flag(AttributeConfig::Value(0)); // EncodedUint32
        let reversed_path = block.flag(AttributeConfig::DiscreteProperty(false));
        let perpendicular_to_path = block.flag(AttributeConfig::DiscreteProperty(false));
        let force_aligment = block.flag(AttributeConfig::DiscreteProperty(false));
        let first_margin = block.flag(AttributeConfig::SimpleProperty(0.));
        let last_margin = block.flag(AttributeConfig::SimpleProperty(0.));

        let result = Self {
            path: block.read(path).unwrap_or(0),
            reversed_path: block.read(reversed_path).unwrap_or(false),
            perpendicular_to_path: block.read(perpendicular_to_path).unwrap_or(false),
            force_alignment: block.read(force_aligment).unwrap_or(false),
            first_margin: block.read(first_margin).unwrap_or(0.),
            last_margin: block.read(last_margin).unwrap_or(0.),
        };
        let input = block.finish();

        log::debug!("parse_TextPathOption => {:?}", result);
        Ok((input, result))
    }
}

#[derive(Debug)]
pub struct TextMoreOption {
    // pub inner: AttributeBlock,
    pub anchor_point_grouping: ParagraphJustification,
    pub grouping_alignment: Point,
}

impl StreamParser for TextMoreOption {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        log::debug!("parse_TextMoreOption <= {} bytes", input.len());

        let mut block = AttributeBlock::new(input);
        let anchor_point_grouping =
            block.flag(AttributeConfig::Value(ParagraphJustification::LeftJustify));
        let grouping_alignment =
            block.flag(AttributeConfig::MultiDimensionProperty(Point::new(0., 0.)));

        let result = Self {
            anchor_point_grouping: block
                .read(anchor_point_grouping)
                .unwrap_or(ParagraphJustification::LeftJustify),
            grouping_alignment: block.read(grouping_alignment).unwrap_or(Point::new(0., 0.)),
        };
        let input = block.finish();

        log::debug!("parse_TextMoreOption => {:?}", result);
        Ok((input, result))
    }
}

/// TextSource ⽂本信息，包含：⽂本，字体，⼤⼩，颜⾊等基础信息。
#[derive(Debug)]
pub struct TextSource {
    // pub inner: AttributeBlock,
    pub source_text: TextDocument,
}

impl StreamParser for TextSource {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        log::debug!("parse_TextSource <= {} bytes", input.len());

        let mut block = AttributeBlock::new(input);
        let source_text = block.flag(AttributeConfig::DiscreteProperty(TextDocument::new())); // ??

        let result = Self {
            source_text: block.read(source_text).unwrap_or(TextDocument::new()), // ??
        };
        let input = block.finish();

        log::debug!("parse_TextSource => {:?}", result);
        Ok((input, result))
    }
}
