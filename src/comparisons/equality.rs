use crate::{
    QuirkSink, SNumber, SValue,
    quirks::{SNumberToFloatQ, SValueToNumberQ},
};

impl SValue {
    /// Compares two Scratch values for equality
    ///
    /// If any of the two values is a non-numeric text
    /// (it can't be parsed as float or int) or a boolean,
    /// both values will be converted to strings.
    /// Those strings will then be compared for equality.
    ///
    /// Otherwise, the numeric equality is used as in [`SNumber::q_eq`].
    pub fn q_eq<Q>(&self, other: &SValue, sink: &mut Q) -> bool
    where
        Q: QuirkSink<SValueToNumberQ> + QuirkSink<SNumberToFloatQ>,
    {
        if (self.is_number() || self.is_bool_but_not_as_text())
            && (other.is_number() || other.is_bool_but_not_as_text())
        {
            let a = self.q_as_number(sink);
            let b = other.q_as_number(sink);
            a.q_eq(&b, sink)
        } else {
            let a = self.as_text();
            let b = other.as_text();
            a == b
        }
    }
}

impl SNumber {
    /// Compares two Scratch numbers for equality
    ///
    /// - `[t]` `-∞`  == `-∞`         → true
    /// - `[f]` `-∞`  == `+∞`         → false
    /// - `[f]` `+∞`  == `-∞`         → false
    /// - `[t]` `+∞`  == `+∞`         → true
    /// - `[t]` `NaN` == `NaN`        → true
    /// - `[f]` `NaN` == number       → false
    /// - `[f]` number == `NaN`       → false
    ///
    /// If both values are *integers*, integer equality is used.
    /// If both values are *finite floating point numbers*,
    /// floating point equality is used.
    ///
    /// `NaN` is only equal to `NaN`.
    ///
    /// (short value in `[]` for easy reference)
    pub fn q_eq<Q>(&self, other: &SNumber, sink: &mut Q) -> bool
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        if let (SNumber::Int(a), SNumber::Int(b)) = (self, other) {
            return a == b;
        }
        let a = self.q_as_float(sink);
        let b = other.q_as_float(sink);

        if a.is_infinite() || b.is_infinite() {
            return a.is_infinite()
                && b.is_infinite()
                && a.is_sign_positive() == b.is_sign_positive();
        }
        if a.is_nan() && b.is_nan() {
            return true;
        }
        if a.is_nan() ^ b.is_nan() {
            return false;
        }
        assert!(a.is_finite() && b.is_finite());
        a == b
    }
}
