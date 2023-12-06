use super::{bits::Bits, parsable::Parsable, parser::StreamParser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeType {
    NotExisted,
    Value,
    FixedValue,
    SimpleProperty,
    DiscreteProperty,
    MultiDimensionProperty,
    SpatialProperty,
    BitFlag,
    Custom,
}

#[derive(Debug, Clone, Copy)]
pub enum AttributeConfig<T> {
    Value(T),
    FixedValue(T),
    SimpleProperty(T),
    DiscreteProperty(T),
    MultiDimensionProperty(T),
    SpatialProperty(T),
    BitFlag(T),
    Custom(T),
}

impl<T> From<&AttributeConfig<T>> for AttributeType {
    fn from(config: &AttributeConfig<T>) -> Self {
        match config {
            AttributeConfig::Value(_) => Self::Value,
            AttributeConfig::FixedValue(_) => Self::FixedValue,
            AttributeConfig::SimpleProperty(_) => Self::SimpleProperty,
            AttributeConfig::DiscreteProperty(_) => Self::DiscreteProperty,
            AttributeConfig::MultiDimensionProperty(_) => Self::MultiDimensionProperty,
            AttributeConfig::SpatialProperty(_) => Self::SpatialProperty,
            AttributeConfig::BitFlag(_) => Self::BitFlag,
            AttributeConfig::Custom(_) => Self::Custom,
        }
    }
}

impl<T> From<AttributeConfig<T>> for AttributeType {
    fn from(config: AttributeConfig<T>) -> Self {
        (&config).into()
    }
}

#[derive(Debug, Default)]
pub struct AttributeFlag {
    pub exist: bool,
    pub animatable: bool,
    pub has_spatial: bool,
}

impl AttributeFlag {
    pub const EXISTED: AttributeFlag = Self {
        exist: true,
        animatable: false,
        has_spatial: false,
    };

    pub const NOT_EXISTED: AttributeFlag = Self {
        exist: false,
        animatable: false,
        has_spatial: false,
    };
}

#[derive(Debug)]
enum AttributeBlockState<'a> {
    Flag(Bits<'a>),
    Content(StreamParser<'a>),
}

#[derive(Debug)]
pub struct AttributeBlock<'a> {
    state: AttributeBlockState<'a>,
}

impl<'a> AttributeBlock<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        AttributeBlock {
            state: AttributeBlockState::Flag(Bits::new(input)),
        }
    }

    pub fn flag(&mut self, r#type: impl Into<AttributeType>) -> (AttributeType, AttributeFlag) {
        let r#type = r#type.into();
        let flag = match &self.state {
            AttributeBlockState::Flag(bits) => match r#type {
                AttributeType::NotExisted => AttributeFlag::NOT_EXISTED,
                AttributeType::FixedValue => AttributeFlag::EXISTED,
                _ => {
                    let mut bits = bits.clone();
                    let flag = self.next_flag(r#type, &mut bits);
                    self.state = AttributeBlockState::Flag(bits);
                    flag
                }
            },
            AttributeBlockState::Content(_) => AttributeFlag::default(),
        };
        (r#type, flag)
    }

    fn next_flag(&self, r#type: AttributeType, bits: &mut Bits<'_>) -> AttributeFlag {
        let mut flag = AttributeFlag::default();

        if let AttributeType::FixedValue = r#type {
            flag.exist = true;
            return flag;
        }

        flag.exist = bits.next();
        if !flag.exist {
            return flag;
        }

        if let AttributeType::Value | AttributeType::BitFlag | AttributeType::Custom = r#type {
            return flag;
        }

        flag.animatable = bits.next();
        if !flag.animatable {
            return flag;
        }
        if let AttributeType::SpatialProperty = r#type {
            return flag;
        }

        flag.has_spatial = bits.next();
        flag
    }

    pub fn read<T>(
        &mut self,
        (r#type, flag): (impl Into<AttributeType>, AttributeFlag),
    ) -> Option<T>
    where
        T: Parsable,
    {
        if let AttributeBlockState::Flag(bits) = &self.state {
            match bits.clone().finish() {
                Ok(parser) => {
                    self.state = AttributeBlockState::Content(parser);
                }
                Err(e) => {
                    log::error!("Error: {:?}", e);
                    return None;
                }
            }
        }
        let parser = match &mut self.state {
            AttributeBlockState::Content(parser) => parser,
            _ => return None,
        };

        let r#type = r#type.into();
        match r#type {
            AttributeType::NotExisted => None,
            AttributeType::BitFlag => T::from_bool(flag.exist),
            AttributeType::FixedValue | AttributeType::Value => {
                if flag.exist {
                    T::parse(parser, ()).ok()
                } else {
                    None
                }
            }
            _ => {
                if flag.exist {
                    if flag.animatable {
                        let key_frames = Vec::<String>::new();
                        T::from_key_frames(key_frames.as_slice())
                    } else {
                        T::parse(parser, ()).ok()
                    }
                } else {
                    None
                }
            }
        }
    }
}
