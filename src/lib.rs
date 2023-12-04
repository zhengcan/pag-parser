pub mod format;
pub mod parse;

use format::{Tag, TagBlock};
use parse::{Parsable, ParseError, Parser, ParserContext, SliceParser};

/// Pag 文件格式
#[derive(Debug)]
pub struct Pag {
    pub header: FileHeader,
    pub tag_block: TagBlock,
}

/// Pag 文件头
#[derive(Debug)]
pub struct FileHeader {
    // pub magic: [u8; 3],
    pub version: u8,
    pub length: u32,
    pub compress_method: i8,
}

impl Parsable for FileHeader {
    fn parse(parser: &mut impl Parser, _ctx: impl ParserContext) -> Result<Self, ParseError> {
        let _ = parser.next_term("PAG")?;
        let version = parser.next_u8()?;
        let length = parser.next_u32()?;
        let compress_method = parser.next_i8()?;
        Ok(Self {
            version,
            length,
            compress_method,
        })
    }
}

#[derive(Debug)]
pub struct PagParser<'a> {
    header: FileHeader,
    inner: SliceParser<'a>,
}

impl<'a> PagParser<'a> {
    const DEFAULT_PAG_VERSION: u8 = 1;

    pub fn new(input: &'a [u8]) -> Result<Self, ParseError> {
        let mut parser = SliceParser::new(input);
        let header = FileHeader::parse(&mut parser, ())?;
        if header.version != Self::DEFAULT_PAG_VERSION {
            return Err(ParseError::UnsupportPagVersion(header.version));
        }
        Ok(Self {
            header,
            inner: parser,
        })
    }

    pub fn next_tag(&mut self) -> Result<Tag, ParseError> {
        Tag::parse(&mut self.inner, ())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{PagParser, ParseError};

    #[test]
    fn test_parse_pag() -> Result<(), ParseError> {
        let _ = env_logger::builder()
            .format_module_path(false)
            .filter_level(log::LevelFilter::Debug)
            .try_init();

        let pag = fs::read("libpag/resources/apitest/complex_test.pag")?;
        log::info!("full length = {} bytes", pag.len());

        let mut parser = PagParser::new(pag.as_slice())?;
        log::info!("header = {:?}", parser.header);
        log::info!("====================");

        while let Ok(tag) = parser.next_tag() {
            log::info!("{:?}", tag);
            log::info!("====================");
        }

        Ok(())
    }
}
