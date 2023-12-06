mod attr;
mod bits;
mod context;
mod error;
mod parsable;
mod parser;
mod types;

pub use attr::{AttributeConfig, AttributeType};
pub use context::ParseContext;
pub use error::ParseError;
pub use parsable::Parsable;
pub use parser::{Parser, StreamParser};
pub use types::*;

use crate::{FileHeader, Pag, Tag};

/// PAG File Parser
#[derive(Debug)]
pub struct PagParser<'a> {
    /// The header of file
    header: FileHeader,
    /// The internal bytes based parser
    inner: StreamParser<'a>,
}

impl<'a> PagParser<'a> {
    const DEFAULT_PAG_VERSION: u8 = 1;

    /// Create new instance
    /// - input: the content of PAG file
    pub fn new(input: &'a [u8]) -> Result<Self, ParseError> {
        let mut parser = StreamParser::new(input);

        // Parse and check file header
        let header = FileHeader::parse(&mut parser, ())?;
        if header.version != Self::DEFAULT_PAG_VERSION {
            return Err(ParseError::UnsupportPagVersion(header.version));
        }

        // Return parser
        Ok(Self {
            header,
            inner: parser,
        })
    }

    /// Parser next tag section
    pub fn next_tag(&mut self) -> Option<Result<Tag, ParseError>> {
        if self.inner.is_empty() {
            None
        } else {
            Some(Tag::parse(&mut self.inner, ()))
        }
    }
}

impl<'a> PagParser<'a> {
    /// Parse whole input to a Pag object
    pub fn parse_all(input: &'a [u8]) -> Result<Pag, ParseError> {
        let mut parser = Self::new(input)?;
        let mut pag = Pag::new(parser.header.clone());
        while let Some(tag) = parser.next_tag() {
            pag.push_tag(tag?);
        }
        Ok(pag)
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
        names.push("tests/pags/12767270.pag");
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
