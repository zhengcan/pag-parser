#[cfg(test)]
mod tests {
    use std::{fs, io};

    use nom::Err;

    use crate::format::{FileHeader, StreamParser, TagBlock};

    #[test]
    fn test_parse_pag() -> io::Result<()> {
        let _ = env_logger::builder()
            .format_module_path(false)
            .filter_level(log::LevelFilter::Debug)
            .try_init();

        let pag = fs::read("libpag/resources/apitest/complex_test.pag")?;
        println!("full length = {} bytes", pag.len());

        let (input, header) = FileHeader::parse(pag.as_slice())
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "parse_file_header"))?;
        println!("header = {:?}", header);

        let (input, tag_block) = TagBlock::parse(input).map_err(|e| {
            match e {
                Err::Incomplete(error) => {
                    log::error!("Incomplete: {:?}", error);
                }
                Err::Error(error) => {
                    log::error!("Error: {:?}", error.code);
                }
                Err::Failure(error) => {
                    log::error!("Failure: {:?}", error.code);
                }
            };
            io::Error::new(io::ErrorKind::Other, "parse_tag_block")
        })?;
        println!("tag_block = {:?}", tag_block);
        println!("remain = {:?} bytes", input);

        Ok(())
    }
}
