use crate::{SNumber, SValue};

impl SValue {
    pub(crate) const NAN: Self = Self::Float(f64::NAN);
    pub(crate) const INFINITY: Self = Self::Float(f64::INFINITY);
    pub(crate) const NEG_INFINITY: Self = Self::Float(f64::NEG_INFINITY);
    pub(crate) const POS_ZERO: Self = Self::Float(0.0);
    pub(crate) const NEG_ZERO: Self = Self::Float(-0.0);
}

impl SNumber {
    pub(crate) const NAN: Self = Self::Float(f64::NAN);
    pub(crate) const INFINITY: Self = Self::Float(f64::INFINITY);
    pub(crate) const NEG_INFINITY: Self = Self::Float(f64::NEG_INFINITY);
    pub(crate) const POS_ZERO: Self = Self::Float(0.0);
    pub(crate) const NEG_ZERO: Self = Self::Float(-0.0);
}
