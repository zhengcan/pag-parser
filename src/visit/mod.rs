use crate::LayerType;

/// 图层信息
pub trait LayerInfo {
    /// 获取图层名称
    fn get_layer_name(&self) -> Option<&str>;
    /// 获取图层类型
    fn get_layer_type(&self) -> LayerType;
}

/// 可遍历
pub trait Traversable {
    /// 遍历图层
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
