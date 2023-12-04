use macros::ParsableEnum;
use pag_parser::parse;

#[derive(Debug, ParsableEnum)]
#[repr(u8)]
pub enum MyEnum {
    Zero,
}

impl From<u8> for MyEnum {
    fn from(_: u8) -> Self {
        Self::Zero
    }
}
