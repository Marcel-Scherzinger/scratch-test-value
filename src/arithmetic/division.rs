use crate::{
    QuirkSink, SNumber, SValue,
    quirks::{SNumberToFloatQ, SValueToNumberQ},
};

impl SValue {
    /// See [`SNumber::q_div_numbers`]
    pub fn q_div_numbers<Q>(&self, other: &SValue, sink: &mut Q) -> SValue
    where
        Q: QuirkSink<SValueToNumberQ> + QuirkSink<SNumberToFloatQ>,
    {
        self.q_as_number(sink)
            .q_div_numbers(&other.q_as_number(sink), sink)
            .svalue()
    }
}

impl SNumber {
    /// Integers are automatically divided as floats (if the remainder != 0) but I think this
    /// shouldn't be considered a quirk as some people would actually like it (?)
    ///
    /// The division rules are not necessarily identical to IEEE 754.
    ///
    /// - n·m / m → n (integer division without remainder)
    ///
    /// If both numbers are finite integers and the division would have a remainder
    /// of 0 the numbers are divided as integers.
    /// Otherwise, the below table applys. p and q are arbitraty finite floats.
    ///
    /// (line / column is shown in cell)
    ///
    /// | `/`   |    | `NaN` | `+∞`  | `-∞`  |  +q  | -q   | `+0`  | `-0`  |
    /// | ----- | -- | ----- | ----- | ----- | ---- | ---- | ----- | ----- |
    /// |       |    |       |       |       |      |      |       |       |
    /// | `NaN` |    | `NaN` | `+0`  | `-0`  | `+0` | `-0` | `NaN` | `NaN` |
    /// | `+∞`  |    | `+∞`  | `NaN` | `NaN` | `+∞` | `-∞` | `+∞`  | `-∞`  |
    /// | `-∞`  |    | `-∞`  | `NaN` | `NaN` | `-∞` | `+∞` | `-∞`  | `+∞`  |
    /// |  +p   |    | `+∞`  | `+0`  | `-0`  |  p/q | -p/q | `+∞`  | `-∞`  |
    /// |  -p   |    | `-∞`  | `-0`  | `+0`  | -p/q |  p/q | `-∞`  | `+∞`  |
    /// | `+0`  |    | `NaN` | `+0`  | `-0`  | `+0` | `-0` | `NaN` | `NaN` |
    /// | `-0`  |    | `NaN` | `-0`  | `+0`  | `-0` | `+0` | `NaN` | `NaN` |
    pub fn q_div_numbers<Q>(&self, other: &SNumber, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        if let (SNumber::Int(a), SNumber::Int(b)) = (self, other)
            && a.checked_rem(*b) == Some(0)
            && let Some(res) = a.checked_div(*b)
        {
            return SNumber::Int(res);
        }

        let a = self.q_as_float(sink);
        let b = other.q_as_float(sink);

        if a.is_nan() && b.is_nan() {
            return SNumber::NAN;
        }

        if (a == 0.0 || a == -0.0) && b.is_nan() {
            return Self::NAN;
        }
        if a.is_nan() && (b == 0.0 || b == -0.0) {
            return Self::NAN;
        }

        if b.is_nan() {
            return if a.is_sign_positive() {
                SNumber::INFINITY
            } else {
                SNumber::NEG_INFINITY
            };
        }
        if a.is_nan() && b.is_infinite() {
            return SNumber::Float(if b.is_sign_positive() { 0.0 } else { -0.0 });
        }
        if a.is_nan() && b.is_finite() {
            return SNumber::Float(if b.is_sign_positive() { 0.0 } else { -0.0 });
        }

        SNumber::Float(a / b)
    }
}
