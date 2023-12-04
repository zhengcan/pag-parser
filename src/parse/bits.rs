use super::parser::SliceParser;

#[derive(Debug, Clone)]
pub struct Bits<'a> {
    buffer: &'a [u8],
    index: usize,
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

    pub fn finish<'b>(self) -> SliceParser<'b>
    where
        'a: 'b,
    {
        let buffer = &self.buffer[(self.index + 7) / 8..];
        SliceParser::new(buffer)
    }
}
