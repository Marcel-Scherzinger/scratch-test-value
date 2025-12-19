use crate::SNumber;

impl SNumber {
    pub fn sqrt(&self) -> SNumber {
        match self {
            Self::Int(i) => {
                if *i < 0 {
                    return SNumber::NAN;
                }
                SNumber::Float((*i as f64).sqrt())
            }
            Self::Float(f) => {
                if f.is_nan() {
                    return SNumber::Int(0);
                }
                if f.is_sign_negative() {
                    return SNumber::NAN;
                }
                SNumber::Float(f.sqrt())
            }
        }
    }
}
