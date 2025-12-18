use crate::{QuirkSink, SNumber, SValue, quirks::SValueToNumberQ};

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
impl SValue {
    pub fn sqrt<Q>(&self, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SValueToNumberQ>,
    {
        self.q_as_number(sink).sqrt()
    }
}
