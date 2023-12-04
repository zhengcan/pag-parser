use std::fmt::Debug;

use crate::format::{LayerType, TagCode};

pub trait ParseContext: Clone {
    fn with_tag_code(&self, tag_code: TagCode) -> impl ParseContext;
    fn with_layer_type(&self, layer_type: LayerType) -> impl ParseContext;
    fn with_alpha(&self, has_alpha: bool) -> impl ParseContext;

    fn parent_code(&self) -> Option<TagCode>;
    fn layer_type(&self) -> Option<LayerType>;
    fn has_alpha(&self) -> bool;
}

#[derive(Debug, Clone)]
struct DefaultParseContext {
    parent_code: Option<TagCode>,
    layer_type: Option<LayerType>,
    has_alpha: bool,
}

impl ParseContext for DefaultParseContext {
    fn with_tag_code(&self, tag_code: TagCode) -> impl ParseContext {
        Self {
            parent_code: Some(tag_code),
            ..self.clone()
        }
    }

    fn with_layer_type(&self, layer_type: LayerType) -> impl ParseContext {
        Self {
            layer_type: Some(layer_type),
            ..self.clone()
        }
    }

    fn with_alpha(&self, has_alpha: bool) -> impl ParseContext {
        Self {
            has_alpha,
            ..self.clone()
        }
    }

    fn parent_code(&self) -> Option<TagCode> {
        self.parent_code
    }

    fn layer_type(&self) -> Option<LayerType> {
        self.layer_type
    }

    fn has_alpha(&self) -> bool {
        self.has_alpha
    }
}

impl ParseContext for () {
    fn with_tag_code(&self, tag_code: TagCode) -> impl ParseContext {
        DefaultParseContext {
            parent_code: Some(tag_code),
            layer_type: None,
            has_alpha: false,
        }
    }

    fn with_layer_type(&self, layer_type: LayerType) -> impl ParseContext {
        DefaultParseContext {
            parent_code: None,
            layer_type: Some(layer_type),
            has_alpha: false,
        }
    }

    fn with_alpha(&self, has_alpha: bool) -> impl ParseContext {
        DefaultParseContext {
            parent_code: None,
            layer_type: None,
            has_alpha,
        }
    }

    fn parent_code(&self) -> Option<TagCode> {
        None
    }

    fn layer_type(&self) -> Option<LayerType> {
        None
    }

    fn has_alpha(&self) -> bool {
        false
    }
}

impl ParseContext for bool {
    fn with_tag_code(&self, tag_code: TagCode) -> impl ParseContext {
        DefaultParseContext {
            parent_code: Some(tag_code),
            layer_type: None,
            has_alpha: false,
        }
    }

    fn with_layer_type(&self, layer_type: LayerType) -> impl ParseContext {
        DefaultParseContext {
            parent_code: None,
            layer_type: Some(layer_type),
            has_alpha: false,
        }
    }

    fn with_alpha(&self, has_alpha: bool) -> impl ParseContext {
        DefaultParseContext {
            parent_code: None,
            layer_type: None,
            has_alpha,
        }
    }

    fn parent_code(&self) -> Option<TagCode> {
        None
    }

    fn layer_type(&self) -> Option<LayerType> {
        None
    }

    fn has_alpha(&self) -> bool {
        *self
    }
}
