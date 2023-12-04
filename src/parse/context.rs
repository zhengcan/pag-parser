use std::fmt::Debug;

use crate::format::{LayerType, TagCode};

pub trait ParserContext: Clone {
    fn with_tag_code(&self, tag_code: TagCode) -> impl ParserContext;
    fn with_layer_type(&self, layer_type: LayerType) -> impl ParserContext;

    fn as_bool(&self) -> bool;
    fn parent_code(&self) -> Option<TagCode>;
    fn layer_type(&self) -> Option<LayerType>;
}

#[derive(Debug, Clone)]
struct DefaultParserContext {
    tag_code: Option<TagCode>,
    layer_type: Option<LayerType>,
}

impl ParserContext for DefaultParserContext {
    fn with_tag_code(&self, tag_code: TagCode) -> impl ParserContext {
        Self {
            tag_code: Some(tag_code),
            ..self.clone()
        }
    }

    fn with_layer_type(&self, layer_type: LayerType) -> impl ParserContext {
        Self {
            layer_type: Some(layer_type),
            ..self.clone()
        }
    }

    fn as_bool(&self) -> bool {
        false
    }

    fn parent_code(&self) -> Option<TagCode> {
        None
    }

    fn layer_type(&self) -> Option<LayerType> {
        self.layer_type
    }
}

impl ParserContext for () {
    fn with_tag_code(&self, tag_code: TagCode) -> impl ParserContext {
        DefaultParserContext {
            tag_code: Some(tag_code),
            layer_type: None,
        }
    }

    fn with_layer_type(&self, layer_type: LayerType) -> impl ParserContext {
        DefaultParserContext {
            tag_code: None,
            layer_type: Some(layer_type),
        }
    }

    fn as_bool(&self) -> bool {
        false
    }

    fn parent_code(&self) -> Option<TagCode> {
        None
    }

    fn layer_type(&self) -> Option<LayerType> {
        None
    }
}

impl ParserContext for bool {
    fn with_tag_code(&self, tag_code: TagCode) -> impl ParserContext {
        DefaultParserContext {
            tag_code: Some(tag_code),
            layer_type: None,
        }
    }

    fn with_layer_type(&self, layer_type: LayerType) -> impl ParserContext {
        DefaultParserContext {
            tag_code: None,
            layer_type: Some(layer_type),
        }
    }

    fn as_bool(&self) -> bool {
        *self
    }

    fn parent_code(&self) -> Option<TagCode> {
        None
    }

    fn layer_type(&self) -> Option<LayerType> {
        None
    }
}
