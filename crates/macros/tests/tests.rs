use macros::Parsable;
use pag_parser::format;

#[derive(Debug, Parsable)]
#[repr(u8)]
pub enum MyEnum {
    Zero,
}

impl From<u8> for MyEnum {
    fn from(_: u8) -> Self {
        Self::Zero
    }
}
