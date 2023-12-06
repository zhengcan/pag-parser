use std::fmt;
use std::ops::Deref;

use concat_idents::concat_idents;

use super::{Parsable, ParseContext, ParseError, Parser};

macro_rules! define_encoded_num {
    ($t:ident, $r:ident) => {
        #[derive(Clone, Copy)]
        pub struct $t($r);

        impl $t {
            pub fn to_u32(&self) -> u32 {
                self.0 as u32
            }

            pub fn to_i32(&self) -> i32 {
                self.0 as i32
            }

            pub fn to_u64(&self) -> u64 {
                self.0 as u64
            }

            pub fn to_i64(&self) -> i64 {
                self.0 as i64
            }

            pub fn to_usize(&self) -> usize {
                self.0 as usize
            }
        }

        impl From<$r> for $t {
            fn from(value: $r) -> Self {
                Self(value)
            }
        }

        impl From<$t> for $r {
            fn from(value: $t) -> Self {
                value.0
            }
        }

        impl PartialEq<$r> for $t {
            fn eq(&self, other: &$r) -> bool {
                self.0 == *other
            }
        }

        impl Deref for $t {
            type Target = $r;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl fmt::Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Parsable for $t {
            fn parse(
                parser: &mut impl Parser,
                _ctx: impl ParseContext,
            ) -> Result<Self, ParseError> {
                concat_idents!(fn_name = next_encoded_, $r {
                    parser.fn_name()
                })
            }
        }
    };
}

define_encoded_num!(EncodedUint32, u32);
define_encoded_num!(EncodedInt32, i32);
define_encoded_num!(EncodedUint64, u64);
define_encoded_num!(EncodedInt64, i64);

pub type Time = EncodedUint64;
