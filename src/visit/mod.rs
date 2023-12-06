use crate::LayerType;

/// Layer info trait
pub trait LayerInfo {
    /// Get layer name
    fn get_layer_name(&self) -> Option<&str>;
    /// Get layer type
    fn get_layer_type(&self) -> LayerType;
}

/// Traversable trait
pub trait Traversable {
    /// Traverse all layers
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

        for entry in fs::read_dir("tests/pags")? {
            let entry = entry?;
            if entry.file_name().to_string_lossy().ends_with(".pag") {
                let buf = fs::read(entry.path())?;
                let pag = PagParser::parse_all(buf.as_slice())?;

                log::info!("{:?}", pag.header);
                pag.traverse_layer(|layer| {
                    log::info!(
                        "name = {:?}, type = {:?}",
                        layer.get_layer_name(),
                        layer.get_layer_type()
                    );
                });
                log::info!("");
            }
        }

        Ok(())
    }
}
