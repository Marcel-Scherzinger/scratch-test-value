use crate::{QuirkSink, SNumber, quirks::SNumberToFloatQ};

impl SNumber {
    pub fn q_exp<Q>(&self, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        SNumber::Float(self.q_as_float_nan_is_zero(sink).exp())
    }

    pub fn q_ln<Q>(&self, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        Self::Float(self.q_as_float_nan_is_zero(sink).ln())
    }

    pub fn q_log10<Q>(&self, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        Self::Float(self.q_as_float_nan_is_zero(sink).log10())
    }

    /// This computes `10^value` where value is the number in `self`
    ///
    /// If the stored number is an integer, the exponent can losslessly
    /// be interpreted as [`u32`] and the result fits into [`i64`]
    /// the integer function [`i64::checked_pow`] is used.
    ///
    /// Otherwise [`f64::powf`] is used, `NaN` will be treated as 0.
    pub fn q_power_of_10<Q>(&self, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        if let Self::Int(i) = self
            && let Ok(exponent) = u32::try_from(*i)
            && let Some(result) = 10_i64.checked_pow(exponent)
        {
            Self::Int(result)
        } else {
            Self::Float(10.0_f64.powf(self.q_as_float_nan_is_zero(sink)))
        }
    }

    #[allow(unused)]
    pub(crate) fn exp(&self) -> SNumber {
        self.q_exp(&mut ())
    }
    #[allow(unused)]
    pub(crate) fn ln(&self) -> SNumber {
        self.q_ln(&mut ())
    }
    #[allow(unused)]
    pub(crate) fn power_of_10(&self) -> SNumber {
        self.q_power_of_10(&mut ())
    }
    #[allow(unused)]
    pub(crate) fn log10(&self) -> SNumber {
        self.q_log10(&mut ())
    }
}

#[cfg(test)]
mod tests {
    use crate::SNumber;

    #[test]
    fn test_ln_special() {
        assert_eq!(SNumber::NEG_INFINITY, SNumber::Float(0.0).ln());
        assert_eq!(SNumber::POS_ZERO, SNumber::Float(1.0).ln());
        assert_eq!(SNumber::NEG_ZERO, SNumber::Float(1.0).ln());
        assert_eq!(SNumber::INFINITY, SNumber::INFINITY.ln());
        assert_eq!(SNumber::NAN, SNumber::NEG_INFINITY.ln());
        assert_eq!(SNumber::NEG_INFINITY, SNumber::NAN.ln());
    }

    #[test]
    fn test_log10_special() {
        assert_eq!(SNumber::NEG_INFINITY, SNumber::Float(0.0).log10());
        assert_eq!(SNumber::POS_ZERO, SNumber::Float(1.0).log10());
        assert_eq!(SNumber::NEG_ZERO, SNumber::Float(1.0).log10());
        assert_eq!(SNumber::INFINITY, SNumber::INFINITY.log10());
        assert_eq!(SNumber::NAN, SNumber::NEG_INFINITY.log10());
        assert_eq!(SNumber::NEG_INFINITY, SNumber::NAN.log10());
    }

    #[test]
    fn test_exp_special() {
        assert_eq!(SNumber::Int(1), SNumber::Float(0.0).exp());
        assert_eq!(SNumber::INFINITY, SNumber::INFINITY.exp());
        assert_eq!(SNumber::POS_ZERO, SNumber::NEG_INFINITY.exp());
        assert_eq!(SNumber::NEG_ZERO, SNumber::NEG_INFINITY.exp());
        assert_eq!(SNumber::Float(1.0), SNumber::NAN.exp());
        assert_eq!(SNumber::Int(1), SNumber::NAN.exp());

        assert_eq!(SNumber::INFINITY, SNumber::Float(10000.0).exp());
    }

    #[test]
    fn test_10_to_special() {
        assert_eq!(SNumber::Int(1), SNumber::Float(0.0).power_of_10());
        assert_eq!(SNumber::INFINITY, SNumber::INFINITY.power_of_10());
        assert_eq!(SNumber::POS_ZERO, SNumber::NEG_INFINITY.power_of_10());
        assert_eq!(SNumber::NEG_ZERO, SNumber::NEG_INFINITY.power_of_10());
        assert_eq!(SNumber::Float(1.0), SNumber::NAN.power_of_10());
        assert_eq!(SNumber::Int(1), SNumber::NAN.power_of_10());
    }
}
