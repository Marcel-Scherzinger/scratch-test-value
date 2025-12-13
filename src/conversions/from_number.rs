use crate::{SNumber, SValue};

impl SNumber {
    pub const fn svalue(self) -> SValue {
        match self {
            Self::Int(i) => SValue::Int(i),
            Self::Float(f) => SValue::Float(f),
        }
    }
}

impl From<SNumber> for SValue {
    fn from(value: SNumber) -> Self {
        value.svalue()
    }
}
