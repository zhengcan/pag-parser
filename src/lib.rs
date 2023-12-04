pub mod format;
pub mod parse;

use format::{Tag, TagBlock};
use parse::{Parsable, ParseContext, ParseError, Parser, SliceParser};

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
    fn parse(parser: &mut impl Parser, _ctx: impl ParseContext) -> Result<Self, ParseError> {
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

    pub fn pag_version(&self) -> u8 {
        self.header.version
    }

    pub fn next_tag(&mut self) -> Option<Result<Tag, ParseError>> {
        if self.inner.is_empty() {
            None
        } else {
            Some(Tag::parse(&mut self.inner, ()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self},
        path::Path,
    };

    use super::{PagParser, ParseError};

    #[test]
    fn test_parse_all_pag() -> Result<(), ParseError> {
        let _ = env_logger::builder()
            .format_module_path(false)
            .filter_level(log::LevelFilter::Info)
            .try_init();

        for entry in fs::read_dir("libpag/resources/apitest")? {
            let entry = entry?;
            if entry.file_name().to_string_lossy().ends_with(".pag") {
                match parse_single(entry.path().as_path()) {
                    Ok(_) => {}
                    Err(e) => {
                        log::warn!("Error: {:?}", e);
                    }
                }
                log::info!("");
            }
        }

        Ok(())
    }

    #[test]
    fn test_parse_single_pag() -> Result<(), ParseError> {
        let _ = env_logger::builder()
            .format_module_path(false)
            .filter_level(log::LevelFilter::Debug)
            .try_init();

        // let name = "libpag/resources/apitest/ImageLayerBounds.pag";
        let mut names = vec![];
        // names.push("tests/12767246.pag");
        names.push("tests/12767270.pag");
        for name in names {
            match parse_single(Path::new(name)) {
                Ok(_) => {}
                Err(e) => {
                    log::warn!("Error: {:?}", e);
                }
            }
        }

        Ok(())
    }

    fn parse_single(path: &Path) -> Result<(), ParseError> {
        let pag = fs::read(path)?;
        log::info!("====================");
        log::info!("{} => {} bytes", path.to_string_lossy(), pag.len());

        let mut parser = PagParser::new(pag.as_slice())?;
        log::info!("header = {:?}", parser.header);
        log::info!("--------------------");

        while let Some(tag) = parser.next_tag() {
            match tag {
                Ok(tag) => {
                    log::info!("{:?}", tag.header);
                }
                Err(e) => {
                    log::error!("{:?}", e);
                }
            }
            log::info!("--------------------");
        }

        Ok(())
    }
}
