use super::{bits::Bits, parsable::Parsable, parser::SliceParser};

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

pub trait AttributeValue
where
    Self: Sized,
{
    fn try_from_bool(value: bool) -> Option<Self> {
        None
    }

    fn try_from_key_frames(key_frames: Vec<String>) -> Option<Self> {
        None
    }
}

impl AttributeValue for f32 {}

impl AttributeValue for u8 {}

impl AttributeValue for u32 {}

impl AttributeValue for u64 {}

impl AttributeValue for bool {
    fn try_from_bool(value: bool) -> Option<Self> {
        Some(value)
    }
}

impl AttributeValue for String {}

#[derive(Debug)]
enum AttributeBlockState<'a> {
    Flag(Bits<'a>),
    Content(SliceParser<'a>),
}

#[derive(Debug)]
pub struct AttributeBlock<'a> {
    state: AttributeBlockState<'a>,
}

impl<'a> AttributeBlock<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        // log::warn!("AttributeBlock: << {:?}", &input[0..16]);
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
        return flag;
    }

    pub fn read<T>(
        &mut self,
        (r#type, flag): (impl Into<AttributeType>, AttributeFlag),
    ) -> Option<T>
    where
        T: Parsable + AttributeValue,
    {
        if let AttributeBlockState::Flag(bits) = &self.state {
            self.state = AttributeBlockState::Content(bits.clone().finish());
        }
        let parser = match &mut self.state {
            AttributeBlockState::Content(parser) => parser,
            _ => return None,
        };

        let r#type = r#type.into();
        match r#type {
            AttributeType::NotExisted => None,
            AttributeType::BitFlag => T::try_from_bool(flag.exist),
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
                        let key_frames = vec![];
                        T::try_from_key_frames(key_frames)
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
