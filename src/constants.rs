use crate::{SNumber, SValue};

impl SValue {
    /// For empty wholes in the shape of a comparison block,
    /// Scratch automatically treats its value as `false`
    pub const fn default_empty_cmp_hole() -> Self {
        Self::Bool(false)
    }
    /// For empty wholes in the shape of an expression block,
    /// Scratch automatically treats its value as the empty string
    pub fn default_empty_expr_hole() -> Self {
        Self::Text("".into())
    }
}

#[allow(unused)]
impl SValue {
    pub(crate) const NAN: Self = Self::Float(f64::NAN);
    pub(crate) const INFINITY: Self = Self::Float(f64::INFINITY);
    pub(crate) const NEG_INFINITY: Self = Self::Float(f64::NEG_INFINITY);
    pub(crate) const POS_ZERO: Self = Self::Float(0.0);
    pub(crate) const NEG_ZERO: Self = Self::Float(-0.0);
}

#[allow(unused)]
impl SNumber {
    pub(crate) const NAN: Self = Self::Float(f64::NAN);
    pub(crate) const INFINITY: Self = Self::Float(f64::INFINITY);
    pub(crate) const NEG_INFINITY: Self = Self::Float(f64::NEG_INFINITY);
    pub(crate) const POS_ZERO: Self = Self::Float(0.0);
    pub(crate) const NEG_ZERO: Self = Self::Float(-0.0);
}
