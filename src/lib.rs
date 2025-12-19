mod arithmetic;
mod comparisons;
mod constants;
mod conversions;
mod math;
mod quirk_sink;
mod rand_range;
mod texts;
mod utils;

#[allow(unused)]
pub mod quirks {
    pub use crate::arithmetic::{IntegerAddWouldFailQ, IntegerSubWouldFailQ};
    pub use crate::conversions::{
        NumberTooBigForIntQ, SNumberToFloatQ, SValueToBoolQ, SValueToFloatQ, SValueToNumberQ,
    };
    pub use crate::quirk_sink::QuirkSink;
    pub use crate::texts::NthLetterOfTextQ;
}

pub(crate) use quirk_sink::QuirkSink;

pub use utils::ARc;

/// This models a numeric Scratch value and is therefore returned by numeric
/// operations or where it is known a value is neither a text nor a boolean.
#[derive(derive_more::Debug, Clone, derive_more::Display, Copy)]
pub enum SNumber {
    #[debug("{_0:?}")]
    Int(i64),
    #[debug("{_0:?}")]
    Float(f64),
}

/// This should model a Scratch value.
/// Scratch treats texts that are non-numeric as the number `0` and also stores numbers
/// inside of arithmetic expressions as texts, at least sometimes.
///
/// So it is useful to have a type that mimics this implicit conversion
/// behaviour.
#[derive(derive_more::Debug, Clone, derive_more::Display)]
pub enum SValue {
    #[debug("{_0:?}")]
    Text(ARc<str>),
    #[debug("{_0:?}")]
    Int(i64),
    #[debug("{_0:?}")]
    Float(f64),
    #[debug("{_0:?}")]
    Bool(bool),
}
