use crate::{QuirkSink, SNumber, SValue, quirks::SValueToNumberQ};

impl SValue {
    // is this needed
    fn range_borders_to<Q>(&self, to: &SValue, sink: &mut Q) -> (SNumber, SNumber)
    where
        Q: QuirkSink<SValueToNumberQ>,
    {
        let a = self.q_as_number(sink);
        let b = to.q_as_number(sink);
        (a, b)
    }
}
