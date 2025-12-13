use crate::{QuirkSink, SNumber, SValue, conversions::SNumberToFloatQ, quirks::SValueToNumberQ};

impl SValue {
    pub fn q_lt<Q>(&self, other: &SValue, sink: &mut Q) -> bool
    where
        Q: QuirkSink<SNumberToFloatQ> + QuirkSink<SValueToNumberQ>,
    {
        if (self.is_number() || self.is_bool()) && (other.is_number() || other.is_bool()) {
            self.q_as_number(sink).q_lt(&other.q_as_number(sink), sink)
        } else {
            self.as_text() < other.as_text()
        }
    }

    /// like [`Self::q_lt`] with swapped arguments
    pub fn q_gt<Q>(&self, other: &SValue, sink: &mut Q) -> bool
    where
        Q: QuirkSink<SNumberToFloatQ> + QuirkSink<SValueToNumberQ>,
    {
        other.q_lt(self, sink)
    }
}

impl SNumber {
    /// - `[t]` `NaN`  < `NaN`           → true
    /// - `[f]` `NaN`  < `±∞`            → false
    /// - `[f]` `NaN`  < number          → false
    /// - `[t]` `-∞`   < `NaN`           → true
    /// - `[f]` `-∞`   < `-∞`            → false
    /// - `[t]` `-∞`   < `+∞`            → true
    /// - `[t]` `-∞`   < number          → true
    /// - `[t]` `+∞`   < `NaN`           → true
    /// - `[f]` `+∞`   < other than NaN  → false
    /// - `[t]` number < `NaN`           → true
    /// - `[t]` number < `+∞`            → true
    /// - `[f]` number < `-∞`            → false
    ///
    /// (short value in `[]` for easy reference)
    pub fn q_lt<Q>(&self, other: &SNumber, sink: &mut Q) -> bool
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        if let (Self::Int(a), Self::Int(b)) = (self, other) {
            a < b
        } else {
            let a = self.q_as_float(sink);
            let b = other.q_as_float(sink);
            if a.is_nan() {
                return if b.is_nan() {
                    true
                } else if b.is_infinite() {
                    false
                } else {
                    assert!(b.is_finite());
                    false
                };
            }
            // a is finite or infinite
            if a.is_infinite() && a.is_sign_negative() {
                return if b.is_nan() {
                    true
                } else if b.is_infinite() {
                    a.is_sign_positive()
                } else {
                    assert!(b.is_finite());
                    true
                };
            } else if a.is_infinite() && a.is_sign_positive() {
                // only NaN is bigger than +∞
                return b.is_nan();
            }
            // a is finite
            if b.is_nan() {
                true
            } else if b.is_infinite() {
                b.is_sign_positive()
            } else {
                a < b
            }
        }
    }
    /// like [`Self::q_lt`] with swapped arguments
    pub fn q_gt<Q>(&self, other: &SNumber, sink: &mut Q) -> bool
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        other.q_lt(self, sink)
    }
}
