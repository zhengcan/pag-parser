use std::ffi::{CStr, CString};
use std::fmt::Debug;

use nom::bytes::complete::{tag, take, take_till, take_until, take_until1};
use nom::error::{self, Error, ParseError};
use nom::number::complete::{le_f32, le_i32, le_i8, le_u16, le_u32, le_u64, le_u8};
use nom::sequence::tuple;
use nom::{IResult, InputIter, InputTake, ToUsize};

use crate::format::{
    AlphaStop, AttributeBlock, AttributeContent, AttributeFlag, Bits, BlendMode, ByteData, Color,
    ColorStop, CompositionAttributes, FileHeader, FontData, FontTables, GradientColor, ImageBytes,
    ImageBytes2, ImageBytes3, ImageReference, ImageTables, LayerAttributes, LayerBlock,
    ParagraphJustification, Point, Ratio, SolidColor, Tag, TagBlock, TagBody, TagCode, TagHeader,
    TextDocument, TextMoreOption, TextPathOption, TextSource, Time, TrackMatteType,
    VectorCompositionBlock, VideoCompositionBlock,
};

pub fn parse_file_header(input: &[u8]) -> IResult<&[u8], FileHeader> {
    let (input, (_, version, length, compress_method)) =
        tuple((tag("PAG"), le_u8, le_u32, le_i8))(input)?;
    let header = FileHeader {
        magic: [b'P', b'A', b'G'],
        version,
        length,
        compress_method,
    };
    Ok((input, header))
}

pub fn parse_tag_block(input: &[u8]) -> IResult<&[u8], TagBlock> {
    log::debug!("parse_tag_block <= {} bytes", input.len());
    let mut tags = vec![];
    let mut input = input;
    loop {
        let (next, tag) = parse_tag(input)?;
        input = next;
        match tag.header.code {
            TagCode::Unknown(_) => log::warn!("tag = {:?}", tag.header),
            _ => log::info!("tag = {:?}", tag.header),
        };
        let code = tag.header.code;
        tags.push(tag);
        if code == TagCode::End {
            break;
        }
    }
    Ok((input, TagBlock { tags }))
}

pub fn parse_tag(input: &[u8]) -> IResult<&[u8], Tag> {
    log::debug!("parse_tag <= {} bytes", input.len());
    let (input, header) = parse_tag_header(input)?;
    let (input, body) = nom::bytes::complete::take(header.length)(input)?;
    let body = match header.code {
        TagCode::End => TagBody::End,
        TagCode::FontTables => {
            let (_, body) = parse_font_tables(body)?;
            TagBody::FontTables(body)
        }
        TagCode::VectorCompositionBlock => {
            let (_, body) = parse_vector_composition_block(body)?;
            TagBody::VectorCompositionBlock(body)
        }
        TagCode::CompositionAttributes => {
            let (_, body) = parse_composition_attributes(body)?;
            TagBody::CompositionAttributes(body)
        }
        TagCode::ImageTables => {
            let (_, body) = parse_image_tables(body)?;
            TagBody::ImageTables(body)
        }
        TagCode::LayerBlock => {
            let (_, body) = parse_layer_block(body)?;
            TagBody::LayerBlock(body)
        }
        TagCode::LayerAttributes => {
            let (_, body) = parse_layer_attributes(body)?;
            TagBody::LayerAttributes(body)
        }
        TagCode::SolidColor => {
            let (_, body) = parse_solid_color(body)?;
            TagBody::SolidColor(body)
        }
        TagCode::TextSource => {
            let (_, body) = parse_text_source(body)?;
            TagBody::TextSource(body)
        }
        TagCode::TextPathOption => {
            let (_, body) = parse_text_path_option(body)?;
            TagBody::TextPathOption(body)
        }
        TagCode::TextMoreOption => {
            let (_, body) = parse_text_more_option(body)?;
            TagBody::TextMoreOption(body)
        }
        TagCode::ImageReference => {
            let (_, body) = parse_image_reference(body)?;
            TagBody::ImageReference(body)
        }
        TagCode::ImageBytes => {
            let (_, body) = parse_image_bytes(body)?;
            TagBody::ImageBytes(body)
        }
        TagCode::ImageBytes2 => {
            let (_, body) = parse_image_bytes2(body)?;
            TagBody::ImageBytes2(body)
        }
        TagCode::ImageBytes3 => {
            let (_, body) = parse_image_bytes3(body)?;
            TagBody::ImageBytes3(body)
        }
        TagCode::VideoCompositionBlock => {
            let (_, body) = parse_video_composition_block(body)?;
            TagBody::VideoCompositionBlock(body)
        }
        _ => TagBody::Raw(ByteData::from(body)),
    };
    Ok((input, Tag { header, body }))
}

pub fn parse_tag_header(input: &[u8]) -> IResult<&[u8], TagHeader> {
    log::debug!("parse_tag_header <= {} bytes", input.len());
    let (mut input, code_and_length) = le_u16(input)?;
    let code = (code_and_length >> 6) as u8;
    let mut length = (code_and_length & 0b0011_1111) as u32;
    if length == 0x3f {
        let (input_new, length_new) = le_u32(input)?;
        input = input_new;
        length = length_new;
    }
    let header = TagHeader {
        code: TagCode::from(code),
        length,
    };
    log::debug!("parse_tag_header => {:?}", header);
    Ok((input, header))
}

pub fn parse_font_tables(input: &[u8]) -> IResult<&[u8], FontTables> {
    log::debug!("parse_font_tables <= {} bytes", input.len());
    let (mut input, count) = parse_encode_u32(input)?;
    let mut font_datas = vec![];
    for _ in 0..count {
        let (next, font_data) = parse_font_data(input)?;
        input = next;
        font_datas.push(font_data);
    }
    let font_tables = FontTables { count, font_datas };
    log::debug!("parse_font_tables => {:?}", font_tables);
    Ok((input, font_tables))
}

pub fn parse_font_data(input: &[u8]) -> IResult<&[u8], FontData> {
    log::debug!("parse_font_data <= {} bytes", input.len());
    let (input, (font_family, font_style)) = tuple((parse_string, parse_string))(input)?;
    let font_data = FontData {
        font_family,
        font_style,
    };
    log::debug!("parse_font_data => {:?}", font_data);
    Ok((input, font_data))
}

pub fn parse_string(input: &[u8]) -> IResult<&[u8], String> {
    log::debug!("parse_string <= {} bytes", input.len());
    let (input, buffer) = take_until("\0")(input)?;
    let string = String::from_utf8_lossy(buffer).to_string();
    log::debug!("parse_string => {:?}", string);
    Ok((&input[1..], string))
}

pub fn parse_vector_composition_block(input: &[u8]) -> IResult<&[u8], VectorCompositionBlock> {
    log::debug!("parse_vector_composition_block <= {} bytes", input.len());
    let (input, (id, tag_block)) = tuple((parse_encode_u32, parse_tag_block))(input)?;
    let block = VectorCompositionBlock { id, tag_block };
    log::debug!("parse_vector_composition_block => {:?}", block);
    Ok((input, block))
}

pub fn parse_image_tables(input: &[u8]) -> IResult<&[u8], ImageTables> {
    log::debug!("parse_image_tables <= {} bytes", input.len());
    let (mut input, count) = parse_encode_i32(input)?;
    let mut images = vec![];
    for _ in 0..count {
        let (next, image) = parse_image_bytes(input)?;
        input = next;
        images.push(image);
    }
    let image_tables = ImageTables { count, images };
    log::debug!("parse_image_tables => {:?}", image_tables);
    Ok((input, image_tables))
}

pub fn parse_solid_color(input: &[u8]) -> IResult<&[u8], SolidColor> {
    log::debug!("parse_solid_color <= {} bytes", input.len());
    let (input, (solid_color, width, height)) =
        tuple((parse_color, parse_encode_i32, parse_encode_i32))(input)?;
    let color = SolidColor {
        solid_color,
        width,
        height,
    };
    log::debug!("parse_solid_color => {:?}", color);
    Ok((input, color))
}

impl ReadFromAttributeContent for TextDocument {
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        parse_text_document(input)
    }
}

pub fn parse_text_source(input: &[u8]) -> IResult<&[u8], TextSource> {
    log::debug!("parse_text_source <= {} bytes", input.len());
    let (input, block) = parse_attribute_block(input)?;

    let mut reader = AttributeBlockReader::new(&block);
    let text_source = TextSource {
        source_text: reader.read().unwrap(), // ??
    };

    log::debug!("parse_text_source => {:?}", text_source);
    Ok((input, text_source))
}

pub fn parse_text_document(input: &[u8]) -> IResult<&[u8], TextDocument> {
    log::debug!("parse_text_document <= {} bytes: {:?}", input.len(), input);
    let bits = Bits::new(input);
    log::debug!("bits = {:?}", bits);
    let input = &input[bits.bytes_count()..];

    let (
        input,
        (
            baseline_shift,
            first_baseline,
            box_text_pos,
            box_text_size,
            fill_color,
            font_size,
            stroke_color,
            stroke_width,
            text,
            justification,
            leading,
            tracking,
            font_id,
        ),
    ) = tuple((
        le_f32,
        le_f32,
        parse_point,
        parse_point,
        parse_color,
        le_f32,
        parse_color,
        le_f32,
        parse_string,
        le_u8,
        le_f32,
        le_f32,
        parse_encode_u32,
    ))(input)?;
    let text_document = TextDocument {
        apply_fill_flag: bits.get(0),
        apply_stroke_flag: bits.get(1),
        box_text_flag: bits.get(2),
        faux_bold_flag: bits.get(3),
        faux_italic_flag: bits.get(4),
        stroke_over_fill_flag: bits.get(5),
        baseline_shift_flag: bits.get(6),
        first_baseline_flag: bits.get(7),
        box_text_pos_flag: bits.get(8),
        box_text_size_flag: bits.get(9),
        fill_color_flag: bits.get(10),
        font_size_flag: bits.get(11),
        stroke_color_flag: bits.get(12),
        stroke_width_flag: bits.get(13),
        text_flag: bits.get(14),
        justification_flag: bits.get(15),
        leading_flag: bits.get(16),
        tracking_flag: bits.get(17),
        has_font_data_flag: bits.get(18),
        // --
        baseline_shift: bits.option(6, baseline_shift),
        first_baseline: bits.option(7, first_baseline),
        box_text_pos: bits.option(8, box_text_pos),
        box_text_size: bits.option(9, box_text_size),
        fill_color: bits.option(10, fill_color),
        font_size: bits.option(11, font_size),
        stroke_color: bits.option(12, stroke_color),
        stroke_width: bits.option(13, stroke_width),
        text: bits.option(14, text),
        justification: bits.option(15, justification),
        leading: bits.option(16, leading),
        tracking: bits.option(17, tracking),
        font_id: bits.option(18, font_id),
    };
    log::debug!("parse_text_document => {:?}", text_document);
    Ok((input, text_document))
}

// pub fn parse_bits<'a, Error: ParseError<&'a [u8]>>(
//     count: usize,
// ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Bits, Error> {
//     move |input: &[u8]| Ok((&input[count..], Bits(&input[0..count])))
// }

pub fn parse_attribute_block(input: &[u8]) -> IResult<&[u8], AttributeBlock> {
    log::debug!("parse_attribute_block <= {} bytes", input.len());

    let mut bits = Bits::new(input);
    log::debug!("bits = {:?}", bits);
    let mut flags = vec![];
    while bits.has_next() {
        let flag = match bits.next() {
            true => match bits.next() {
                true => match bits.next() {
                    true => AttributeFlag::ExistedAnimatableSpatial,
                    false => AttributeFlag::ExistedAnimatable,
                },
                false => AttributeFlag::Existed,
            },
            false => AttributeFlag::NotExisted,
        };
        flags.push(flag);
    }
    let input = &input[bits.bytes_count()..];

    let mut contents = vec![];
    for flag in &flags {
        match flag {
            AttributeFlag::Existed => {
                contents.push(AttributeContent {});
            }
            AttributeFlag::ExistedAnimatable => {
                contents.push(AttributeContent {});
            }
            AttributeFlag::ExistedAnimatableSpatial => {
                contents.push(AttributeContent {});
            }
            AttributeFlag::NotExisted => {
                // skip
            }
        }
    }

    let block = AttributeBlock {
        flags,
        buffer: Vec::from(input),
        contents,
    };
    log::debug!("parse_attribute_block => {:?}", block);
    Ok((input, block))
}

pub fn parse_text_path_option(input: &[u8]) -> IResult<&[u8], TextPathOption> {
    log::debug!("parse_text_path_option <= {} bytes", input.len());
    let (input, block) = parse_attribute_block(input)?;

    let mut reader = AttributeBlockReader::new(&block);
    let text_path_option = TextPathOption {
        path: reader.read().unwrap_or(0),
        reversed_path: reader.read().unwrap_or(false),
        perpendicular_to_path: reader.read().unwrap_or(false),
        force_alignment: reader.read().unwrap_or(false),
        first_margin: reader.read().unwrap_or(0.),
        last_margin: reader.read().unwrap_or(0.),
    };

    // let (
    //     input,
    //     (path, reversed_path, perpendicular_to_path, force_alignment, first_margin, last_margin),
    // ) = tuple((
    //     parse_encode_u32,
    //     parse_bool,
    //     parse_bool,
    //     parse_bool,
    //     le_f32,
    //     le_f32,
    // ))(input)?;
    // let text_path_option = TextPathOption {
    //     path,
    //     reversed_path,
    //     perpendicular_to_path,
    //     force_alignment,
    //     first_margin,
    //     last_margin,
    // };
    log::debug!("parse_text_path_option => {:?}", text_path_option);
    Ok((input, text_path_option))
}

impl ReadFromAttributeContent for ParagraphJustification {
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        parse_enum(input)
    }
}

impl ReadFromAttributeContent for Point {
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        parse_point(input)
    }
}

pub fn parse_text_more_option(input: &[u8]) -> IResult<&[u8], TextMoreOption> {
    log::debug!("parse_text_more_option <= {} bytes", input.len());
    let (input, block) = parse_attribute_block(input)?;

    let mut reader = AttributeBlockReader::new(&block);
    let text_more_option = TextMoreOption {
        anchor_point_grouping: reader.read().unwrap_or(ParagraphJustification::LeftJustify),
        grouping_alignment: reader.read().unwrap_or_else(|| Point { x: 0., y: 0. }),
    };

    // let (input, (anchor_point_grouping, grouping_alignment)) =
    //     tuple((parse_enum, parse_point))(input)?;
    // let text_more_option = TextMoreOption {
    //     anchor_point_grouping,
    //     grouping_alignment,
    // };
    log::debug!("parse_text_more_option => {:?}", text_more_option);
    Ok((input, text_more_option))
}

pub fn parse_bool(input: &[u8]) -> IResult<&[u8], bool> {
    let (input, value) = le_u8(input)?;
    Ok((input, value > 0))
}

pub fn parse_image_reference(input: &[u8]) -> IResult<&[u8], ImageReference> {
    log::debug!("parse_image_reference <= {} bytes", input.len());
    let (input, id) = parse_encode_u32(input)?;
    let image_reference = ImageReference { id };
    log::debug!("parse_image_reference => {:?}", image_reference);
    Ok((input, image_reference))
}

pub fn parse_image_bytes(input: &[u8]) -> IResult<&[u8], ImageBytes> {
    log::debug!("parse_image_bytes <= {} bytes", input.len());
    let (input, (id, file_bytes)) = tuple((parse_encode_u32, parse_byte_data))(input)?;
    let image_bytes = ImageBytes { id, file_bytes };
    log::debug!("parse_image_bytes => {:?}", image_bytes);
    Ok((input, image_bytes))
}

pub fn parse_layer_block(input: &[u8]) -> IResult<&[u8], LayerBlock> {
    log::debug!("parse_layer_block <= {} bytes", input.len());
    let (input, (r#type, id, tag_block)) =
        tuple((parse_enum, parse_encode_u32, parse_tag_block))(input)?;
    let block = LayerBlock {
        r#type,
        id,
        tag_block,
    };
    log::debug!("parse_layer_block => {:?}", block);
    Ok((input, block))
}

pub struct AttributeBlockReader<'a> {
    flags: &'a [AttributeFlag],
    buffer: &'a [u8],
    index: usize,
}

pub trait ReadFromAttributeContent {
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}

impl<'a> AttributeBlockReader<'a> {
    pub fn new<'b>(block: &'b AttributeBlock) -> Self
    where
        'b: 'a,
    {
        Self {
            flags: &block.flags,
            buffer: &block.buffer,
            index: 0,
        }
    }

    pub fn read<T>(&mut self) -> Option<T>
    where
        T: ReadFromAttributeContent,
    {
        let current = self.index;
        self.index += 1;

        let input = self.buffer;
        match self.flags.get(current) {
            Some(flag) => {
                //
                match T::read(input) {
                    Ok((input, content)) => {
                        self.buffer = input;
                        Some(content)
                    }
                    Err(_) => None,
                }
            }
            Some(AttributeFlag::NotExisted) | None => None,
        }
    }

    // pub fn read_or<T>(&mut self, default: T) -> T
    // where
    //     T: ReadFromAttributeContent,
    // {
    //     self.read().unwrap_or(default)
    // }

    // pub fn read_or_default<T>(&mut self) -> T
    // where
    //     T: ReadFromAttributeContent + Default,
    // {
    //     self.read().unwrap_or_default()
    // }

    // pub fn read_or_else<T, F>(&mut self, f: F) -> T
    // where
    //     T: ReadFromAttributeContent,
    //     F: FnOnce() -> T,
    // {
    //     self.read().unwrap_or_else(f)
    // }
}

impl ReadFromAttributeContent for bool {
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        parse_bool(input)
    }
}

impl ReadFromAttributeContent for u32 {
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        le_u32(input)
    }
}

impl ReadFromAttributeContent for u64 {
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        le_u64(input)
    }
}

impl ReadFromAttributeContent for f32 {
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        le_f32(input)
    }
}

impl ReadFromAttributeContent for Ratio {
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        parse_ratio(input)
    }
}

impl ReadFromAttributeContent for BlendMode {
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        parse_enum(input)
    }
}

impl ReadFromAttributeContent for TrackMatteType {
    fn read(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        parse_enum(input)
    }
}

pub fn parse_layer_attributes(input: &[u8]) -> IResult<&[u8], LayerAttributes> {
    log::debug!("parse_layer_attributes <= {} bytes", input.len());
    let (input, block) = parse_attribute_block(input)?;

    let mut reader = AttributeBlockReader::new(&block);
    let attributes = LayerAttributes {
        is_active: reader.read().unwrap_or(true),
        auto_orientation: reader.read().unwrap_or_default(),
        parent: reader.read().unwrap_or(0),
        stretch: reader.read().unwrap_or_else(|| Ratio {
            numerator: 1,
            denominator: 1,
        }),
        start_time: reader.read().unwrap_or(0),
        blend_mode: reader.read().unwrap_or(BlendMode::Normal),
        track_matte_type: reader.read().unwrap_or(TrackMatteType::None),
        time_remap: reader.read().unwrap_or(0.),
        duration: reader.read().unwrap_or(0),
    };

    log::debug!("parse_layer_attributes => {:?}", attributes);
    Ok((input, attributes))
}

pub fn parse_ratio(input: &[u8]) -> IResult<&[u8], Ratio> {
    log::debug!("parse_ratio <= {} bytes", input.len());
    let (input, (numerator, denominator)) = tuple((parse_encode_i32, parse_encode_u32))(input)?;
    let ratio = Ratio {
        numerator,
        denominator,
    };
    log::debug!("parse_ratio => {:?}", ratio);
    Ok((input, ratio))
}

pub fn parse_point(input: &[u8]) -> IResult<&[u8], Point> {
    log::debug!("parse_point <= {} bytes", input.len());
    let (input, (x, y)) = tuple((le_f32, le_f32))(input)?;
    let point = Point { x, y };
    log::debug!("parse_point => {:?}", point);
    Ok((input, point))
}

pub fn parse_time(input: &[u8]) -> IResult<&[u8], Time> {
    log::debug!("parse_time <= {} bytes", input.len());
    let (input, time) = parse_encode_u64(input)?;
    log::debug!("parse_time => {:?}", time);
    Ok((input, time))
}

pub fn parse_alpha_stop(input: &[u8]) -> IResult<&[u8], AlphaStop> {
    log::debug!("parse_alpha_stop <= {} bytes", input.len());
    let (input, (position, midpoint, opacity)) = tuple((le_u16, le_u16, le_u8))(input)?;
    let stop = AlphaStop {
        position,
        midpoint,
        opacity,
    };
    log::debug!("parse_alpha_stop => {:?}", stop);
    Ok((input, stop))
}

pub fn parse_color_stop(input: &[u8]) -> IResult<&[u8], ColorStop> {
    log::debug!("parse_color_stop <= {} bytes", input.len());
    let (input, (position, midpoint, color)) = tuple((le_u16, le_u16, parse_color))(input)?;
    let stop = ColorStop {
        position,
        midpoint,
        color,
    };
    log::debug!("parse_color_stop => {:?}", stop);
    Ok((input, stop))
}

pub fn parse_gradient_color(input: &[u8]) -> IResult<&[u8], GradientColor> {
    log::debug!("parse_gradient_color <= {} bytes", input.len());
    let (mut input, (alpha_count, color_count)) = tuple((le_u32, le_u32))(input)?;

    let mut alpha_stop_list = vec![];
    for _ in 0..alpha_count {
        let (next, stop) = parse_alpha_stop(input)?;
        input = next;
        alpha_stop_list.push(stop);
    }

    let mut color_stop_list = vec![];
    for _ in 0..color_count {
        let (next, stop) = parse_color_stop(input)?;
        input = next;
        color_stop_list.push(stop);
    }

    let color = GradientColor {
        alpha_count,
        color_count,
        alpha_stop_list,
        color_stop_list,
    };
    log::debug!("parse_gradient_color => {:?}", color);
    Ok((input, color))
}

pub fn parse_enum<T: From<u8> + Debug>(input: &[u8]) -> IResult<&[u8], T> {
    log::debug!("parse_enum <= {} bytes", input.len());
    let (input, value) = le_u8(input)?;
    let value = T::from(value);
    log::debug!("parse_enum => {:?}", value);
    Ok((input, value))
}

pub fn parse_image_bytes2(input: &[u8]) -> IResult<&[u8], ImageBytes2> {
    log::debug!("parse_image_bytes2 <= {} bytes", input.len());
    let (input, (id, file_bytes, scale_factor)) =
        tuple((parse_encode_u32, parse_byte_data, le_f32))(input)?;
    let image_bytes = ImageBytes2 {
        id,
        file_bytes,
        scale_factor,
    };
    log::debug!("parse_image_bytes2 => {:?}", image_bytes);
    Ok((input, image_bytes))
}

pub fn parse_image_bytes3(input: &[u8]) -> IResult<&[u8], ImageBytes3> {
    log::debug!("parse_image_bytes3 <= {} bytes", input.len());
    let (input, (id, file_bytes, scale_factor, width, height, anchor_x, anchor_y)) = tuple((
        parse_encode_u32,
        parse_byte_data,
        le_f32,
        parse_encode_i32,
        parse_encode_i32,
        parse_encode_i32,
        parse_encode_i32,
    ))(input)?;
    let image_bytes = ImageBytes3 {
        id,
        file_bytes,
        scale_factor,
        width,
        height,
        anchor_x,
        anchor_y,
    };
    log::debug!("parse_image_bytes3 => {:?}", image_bytes);
    Ok((input, image_bytes))
}

pub fn parse_byte_data(input: &[u8]) -> IResult<&[u8], ByteData> {
    log::debug!("parse_byte_data <= {} bytes", input.len());
    let (input, length) = parse_encode_u32(input)?;
    let (input, data) = take(length)(input)?;
    let byte_data = ByteData {
        length,
        data: Vec::from(data),
    };
    log::debug!("parse_byte_data => {:?}", byte_data);
    Ok((input, byte_data))
}

pub fn parse_video_composition_block(input: &[u8]) -> IResult<&[u8], VideoCompositionBlock> {
    log::debug!("parse_video_composition_block <= {} bytes", input.len());
    let (input, (id, has_alpha, composition_attributes, tag_block)) = tuple((
        parse_encode_u32,
        parse_bool,
        parse_composition_attributes,
        parse_tag_block,
    ))(input)?;
    let block = VideoCompositionBlock {
        id,
        has_alpha,
        composition_attributes,
        tag_block,
    };
    log::debug!("parse_video_composition_block => {:?}", block);
    Ok((input, block))
}

pub fn parse_composition_attributes(input: &[u8]) -> IResult<&[u8], CompositionAttributes> {
    log::debug!("parse_composition_attributes <= {} bytes", input.len());
    let (input, (width, height, duration, frame_rate, background_color)) = tuple((
        parse_encode_i32,
        parse_encode_i32,
        parse_encode_u64,
        le_f32,
        parse_color,
    ))(input)?;
    let attributes = CompositionAttributes {
        width,
        height,
        duration,
        frame_rate,
        background_color,
    };
    log::debug!("parse_composition_attributes => {:?}", attributes);
    Ok((input, attributes))
}

pub fn parse_encode_u32(input: &[u8]) -> IResult<&[u8], u32> {
    let mut input = input;
    let mut value = 0u32;
    for i in (0..32).step_by(7) {
        let (next, byte) = le_u8(input)?;
        input = next;
        value |= ((byte & 0x7f) as u32) << i;
        if (byte & 0x80) == 0 {
            break;
        }
    }
    Ok((input, value))
}

pub fn parse_encode_i32(input: &[u8]) -> IResult<&[u8], i32> {
    let (input, value) = parse_encode_u32(input)?;
    Ok((input, value as i32))
}

pub fn parse_encode_u64(input: &[u8]) -> IResult<&[u8], u64> {
    let mut input = input;
    let mut value = 0u64;
    for i in (0..64).step_by(7) {
        let (next, byte) = le_u8(input)?;
        input = next;
        value |= ((byte & 0x7f) as u64) << i;
        if (byte & 0x80) == 0 {
            break;
        }
    }
    Ok((input, value))
}

pub fn parse_encode_i64(input: &[u8]) -> IResult<&[u8], i64> {
    let (input, value) = parse_encode_u64(input)?;
    Ok((input, value as i64))
}

pub fn parse_color(input: &[u8]) -> IResult<&[u8], Color> {
    log::debug!("parse_color <= {} bytes", input.len());
    let (input, (red, green, blue)) = tuple((le_u8, le_u8, le_u8))(input)?;
    let color = Color { red, green, blue };
    log::debug!("parse_color => {:?}", color);
    Ok((input, color))
}

#[cfg(test)]
mod tests {
    use std::{fs, io};

    use crate::parser::{parse_file_header, parse_tag_block};

    #[test]
    fn test_parse_pag() -> io::Result<()> {
        let _ = env_logger::builder()
            .format_module_path(false)
            .filter_level(log::LevelFilter::Debug)
            .try_init();

        let pag = fs::read("libpag/resources/apitest/video_sequence_test.pag")?;
        println!("full length = {} bytes", pag.len());

        let (input, header) = parse_file_header(pag.as_slice())
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "parse_file_header"))?;
        println!("header = {:?}", header);

        let (input, tag_block) = parse_tag_block(input)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "parse_tag_block"))?;
        println!("tag_block = {:?}", tag_block);
        println!("remain = {:?} bytes", input);

        Ok(())
    }
}
