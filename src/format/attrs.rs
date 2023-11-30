use super::StreamParser;

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

    pub fn get(&self, index: usize) -> bool {
        let (i, j) = (index / 8, index % 8);
        // log::error!(
        //     "i={i}, j={j}, b={}, r={}",
        //     self.buffer[i],
        //     (self.buffer[i] & (1 << j)) != 0
        // );
        if i >= self.buffer.len() {
            false
        } else {
            let byte = self.buffer[i];
            (byte & (1 << j)) != 0
        }
    }

    pub fn option<T>(&self, index: usize, value: T) -> Option<T> {
        match self.get(index) {
            true => Some(value),
            false => None,
        }
    }

    pub fn finish(self) -> &'a [u8] {
        &self.buffer[(self.index + 7) / 8..]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AttributeType {
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

#[derive(Debug)]
enum AttributeBlockState<'a> {
    Flag(Bits<'a>),
    Content,
}

#[derive(Debug)]
pub struct AttributeBlock<'a> {
    buffer: &'a [u8],
    state: AttributeBlockState<'a>,
}

impl<'a> AttributeBlock<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        // log::warn!("AttributeBlock: << {:?}", &input[0..16]);
        AttributeBlock {
            buffer: input,
            state: AttributeBlockState::Flag(Bits::new(input)),
        }
    }

    // fn next_bit(&mut self) -> bool {
    //     match self.state {
    //         AttributeBlockState::Flag(bit_index) => {
    //             self.state = AttributeBlockState::Flag(bit_index + 1);
    //             self.get_bit(bit_index)
    //         }
    //         AttributeBlockState::Content => false,
    //     }
    // }

    // fn get_bit(&self, index: usize) -> bool {
    //     let (i, j) = (index / 8, index % 8);
    //     if i >= self.buffer.len() {
    //         false
    //     } else {
    //         let byte = self.buffer[i];
    //         (byte & (1 << j)) != 0
    //     }
    // }

    pub fn flag(&mut self, r#type: impl Into<AttributeType>) -> (AttributeType, AttributeFlag) {
        let r#type = r#type.into();
        let flag = match &self.state {
            AttributeBlockState::Flag(bits) => {
                let mut bits = bits.clone();
                let flag = self.next_flag(r#type, &mut bits);
                self.state = AttributeBlockState::Flag(bits);
                flag
            }
            AttributeBlockState::Content => AttributeFlag::default(),
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

    pub fn read<T>(&mut self, flag: (impl Into<AttributeType>, AttributeFlag)) -> Option<T>
    where
        T: StreamParser,
    {
        if let AttributeBlockState::Flag(bits) = &self.state {
            self.buffer = bits.clone().finish();
            self.state = AttributeBlockState::Content;
        }

        let r#type = flag.0.into();
        let flag = flag.1;

        match r#type {
            AttributeType::BitFlag => T::try_from_bool(flag.exist),
            AttributeType::FixedValue | AttributeType::Value => {
                if flag.exist {
                    self.parse()
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
                        self.parse()
                    }
                } else {
                    None
                }
            }
        }
    }

    fn parse<T>(&mut self) -> Option<T>
    where
        T: StreamParser,
    {
        match T::parse(self.buffer) {
            Ok((input, value)) => {
                self.buffer = input;
                Some(value)
            }
            Err(e) => {
                log::error!("e = {:?}", e);
                None
            }
        }
    }

    pub fn finish(self) -> &'a [u8] {
        self.buffer
    }
}
