use std::{cmp::min, fmt::Debug, num::NonZeroUsize};

use super::{parser::SliceParser, ParseError};

#[derive(Clone)]
pub struct Bits<'a> {
    buffer: &'a [u8],
    index: usize,
}

impl<'a> Debug for Bits<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let head = &self.buffer[0..min(16, self.buffer.len())];
        let current = &self.buffer[self.index..min(self.index + 16, self.buffer.len())];
        f.debug_struct("Bits")
            .field("head", &head)
            .field("index", &self.index)
            .field("current", &current)
            .finish()
    }
}

impl<'a> Bits<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, index: 0 }
    }

    pub fn next(&mut self) -> bool {
        let index = self.index;
        self.index += 1;
        self.get(index)
    }

    fn get(&self, index: usize) -> bool {
        let (i, j) = (index / 8, index % 8);
        if i >= self.buffer.len() {
            false
        } else {
            let byte = self.buffer[i];
            (byte & (1 << j)) != 0
        }
    }

    pub fn finish<'b>(self) -> Result<SliceParser<'b>, ParseError>
    where
        'a: 'b,
    {
        let offset = (self.index + 7) / 8;
        if offset > self.buffer.len() {
            return Err(ParseError::Incomplete(nom::Needed::Size(
                NonZeroUsize::new(offset - self.buffer.len()).unwrap(),
            )));
        }
        let buffer = &self.buffer[offset..];
        Ok(SliceParser::new(buffer))
    }
}
