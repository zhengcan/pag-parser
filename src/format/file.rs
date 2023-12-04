use crate::{
    parse::{EncodedInt64, Parsable, ParseContext, ParseError, Parser},
    visit::{LayerInfo, Traversable},
    Tag, TagBlock,
};

/// Pag 文件格式
#[derive(Debug)]
pub struct Pag {
    pub header: FileHeader,
    pub tag_block: TagBlock,
}

impl Pag {
    pub fn new(header: FileHeader) -> Self {
        Self {
            header,
            tag_block: TagBlock::default(),
        }
    }

    pub fn push_tag(&mut self, tag: Tag) {
        self.tag_block.push(tag);
    }
}

impl Traversable for Pag {
    fn traverse_layer<F>(&self, visitor: F)
    where
        F: Fn(&dyn LayerInfo) + Clone,
    {
        self.tag_block.traverse_layer(visitor);
    }
}

/// Pag 文件头
#[derive(Debug, Clone)]
pub struct FileHeader {
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
pub struct FileAttributes {
    pub timestamp: EncodedInt64,
    pub plugin_version: String,
    pub ae_version: String,
    pub system_version: String,
    pub author: String,
    pub scene: String,
    pub warnings: Vec<String>,
}

impl Parsable for FileAttributes {
    fn parse(parser: &mut impl Parser, _ctx: impl ParseContext) -> Result<Self, ParseError> {
        let timestamp = parser.next_encoded_i64()?;
        let plugin_version = parser.next_string()?;
        let ae_version = parser.next_string()?;
        let system_version = parser.next_string()?;
        let author = parser.next_string()?;
        let scene = parser.next_string()?;

        let mut warnings = vec![];
        let warning_count = parser.next_encoded_u32()?;
        for _ in 0..warning_count.into() {
            warnings.push(parser.next_string()?);
        }

        Ok(Self {
            timestamp,
            plugin_version,
            ae_version,
            system_version,
            author,
            scene,
            warnings,
        })
    }
}
