use std::fmt::Debug;

use nom::{
    bytes::complete::take_until,
    number::complete::{le_f32, le_i32, le_i64, le_u32, le_u64, le_u8},
    IResult,
};

pub fn parse_bool(input: &[u8]) -> IResult<&[u8], bool> {
    let (input, value) = le_u8(input)?;
    Ok((input, value > 0))
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
    let num = (value >> 1) as i32;
    let value = match (value & 1) > 0 {
        true => -num,
        false => num,
    };
    Ok((input, value))
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
    let num = (value >> 1) as i64;
    let value = match (value & 1) > 0 {
        true => -num,
        false => num,
    };
    Ok((input, value))
}

pub type Time = u64;

pub fn parse_time(input: &[u8]) -> IResult<&[u8], Time> {
    parse_encode_u64(input)
}

// impl StreamParser for bool {
//     fn parse(input: &[u8]) -> IResult<&[u8], Self> {
//         parse_bool(input)
//     }

//     fn try_from_bool(value: bool) -> Option<Self> {
//         Some(value)
//     }
// }

// impl StreamParser for u8 {
//     fn parse(input: &[u8]) -> IResult<&[u8], Self>
//     where
//         Self: Sized,
//     {
//         le_u8(input)
//     }
// }

// impl StreamParser for u32 {
//     fn parse(input: &[u8]) -> IResult<&[u8], Self>
//     where
//         Self: Sized,
//     {
//         le_u32(input)
//     }
// }

// impl StreamParser for i32 {
//     fn parse(input: &[u8]) -> IResult<&[u8], Self> {
//         le_i32(input)
//     }
// }

// impl StreamParser for u64 {
//     fn parse(input: &[u8]) -> IResult<&[u8], Self> {
//         le_u64(input)
//     }
// }

// impl StreamParser for i64 {
//     fn parse(input: &[u8]) -> IResult<&[u8], Self> {
//         le_i64(input)
//     }
// }

// impl StreamParser for f32 {
//     fn parse(input: &[u8]) -> IResult<&[u8], Self> {
//         le_f32(input)
//     }
// }

pub fn parse_enum<T: From<u8> + Debug>(input: &[u8]) -> IResult<&[u8], T> {
    // log::debug!("parse_enum <= {} bytes", input.len());
    let (input, value) = le_u8(input)?;
    let value = T::from(value);
    // log::debug!("parse_enum => {:?}", value);
    Ok((input, value))
}

pub fn parse_string(input: &[u8]) -> IResult<&[u8], String> {
    // log::debug!("parse_string <= {} bytes", input.len());
    let (input, buffer) = take_until("\0")(input)?;
    let value = String::from_utf8_lossy(buffer).to_string();
    // log::debug!("parse_string => {:?}", string);
    Ok((&input[1..], value))
}

// impl StreamParser for String {
//     fn parse(input: &[u8]) -> IResult<&[u8], Self> {
//         parse_string(input)
//     }
// }
