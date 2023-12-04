use crate::LayerType;

pub trait LayerInfo {
    fn get_name(&self) -> Option<&str>;
    fn get_layer_type(&self) -> LayerType;
}

pub trait Traversable {
    fn traverse_layer<F>(&self, visitor: F)
    where
        F: Fn(&dyn LayerInfo) + Clone;
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::parse::{PagParser, ParseError};

    use super::Traversable;

    #[test]
    fn test_traverse_layer() -> Result<(), ParseError> {
        let _ = env_logger::builder()
            .format_module_path(false)
            .filter_level(log::LevelFilter::Info)
            .try_init();

        let name = "tests/12767246.pag";
        let buf = fs::read(name)?;
        let pag = PagParser::parse_all(buf.as_slice())?;

        log::info!("{:?}", pag.header);
        pag.traverse_layer(|layer| {
            log::info!(
                "name = {:?}, type = {:?}",
                layer.get_name(),
                layer.get_layer_type()
            );
        });

        Ok(())
    }
}
