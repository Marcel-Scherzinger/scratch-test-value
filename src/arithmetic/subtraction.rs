use crate::{
    QuirkSink, SNumber, SValue,
    quirks::{SNumberToFloatQ, SValueToNumberQ},
};

impl SValue {
    /// See [`SNumber::q_sub_numbers`]
    pub fn q_sub_numbers<Q>(&self, other: &SValue, sink: &mut Q) -> SValue
    where
        Q: QuirkSink<SValueToNumberQ> + QuirkSink<SNumberToFloatQ>,
    {
        self.q_as_number(sink)
            .q_sub_numbers(&other.q_as_number(sink), sink)
            .svalue()
    }
}

impl SNumber {
    /// If both numbers are finite integers numbers are subtracted as integers
    /// if the result fits into an [`i64`].
    ///
    /// Otherwise, the below table applys. p and q are arbitraty finite floats.
    ///
    /// (line / column is shown in cell)
    ///
    /// | `-`   |    | `NaN` | `+∞`  | `-∞`  |  +q     |  -q     | `+0`  | `-0`  |
    /// | ----- | -- | ----- | ----- | ----- | ------- | ------- | ----- | ----- |
    /// |       |    |       |       |       |         |         |       |       |
    /// | `NaN` |    |  `0`  | `-∞`  | `+∞`  |  -q     |  +q     | `-0`  | `+0`  |
    /// | `+∞`  |    | `+∞`  | `NaN` | `+∞`  | `+∞`    | `+∞`    | `+∞`  | `-∞`  |
    /// | `-∞`  |    | `-∞`  | `-∞`  | `NaN` | `-∞`    | `-∞`    | `-∞`  | `-∞`  |
    /// |  +p   |    |  +p   | `-∞`  | `+∞`  | +p-(+q) | +p-(-q) |  +p   |  +p   |
    /// |  -p   |    |  -p   | `-∞`  | `+∞`  | -p-(+q) | -p-(-q) |  -p   |  -p   |
    /// | `+0`  |    | `+0`  | `-∞`  | `+∞`  |  -q     |  +q     |  `0`  |  `0`  |
    /// | `-0`  |    | `-0`  | `-∞`  | `+∞`  |  -q     |  +q     |  `0`  |  `0`  |
    pub fn q_sub_numbers<Q>(&self, other: &SNumber, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        todo!()
    }
}
