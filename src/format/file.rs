use crate::parse::{EncodedInt64, Parsable, ParseContext, ParseError, Parser};

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
