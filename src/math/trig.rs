use crate::{QuirkSink, SNumber, quirks::SNumberToFloatQ};

impl SNumber {
    pub fn q_cos<Q>(&self, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        SNumber::Float(self.q_as_float_nan_is_zero(sink).cos())
    }
    pub fn q_sin<Q>(&self, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        SNumber::Float(self.q_as_float_nan_is_zero(sink).sin())
    }
    pub fn q_tan<Q>(&self, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        SNumber::Float(self.q_as_float_nan_is_zero(sink).tan())
    }
    pub fn q_acos<Q>(&self, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        SNumber::Float(self.q_as_float_nan_is_zero(sink).acos())
    }
    pub fn q_asin<Q>(&self, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        SNumber::Float(self.q_as_float_nan_is_zero(sink).asin())
    }
    pub fn q_atan<Q>(&self, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        SNumber::Float(self.q_as_float_nan_is_zero(sink).atan())
    }
}

#[cfg(test)]
mod tests {
    use crate::SNumber;

    #[test]
    fn test_special_trig() {
        for s in [SNumber::INFINITY, SNumber::NEG_INFINITY] {
            assert_eq!(SNumber::NAN, s.q_cos(&mut ()));
            assert_eq!(SNumber::NAN, s.q_sin(&mut ()));
            assert_eq!(SNumber::NAN, s.q_tan(&mut ()));
        }
    }
}
