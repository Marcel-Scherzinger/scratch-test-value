use crate::{QuirkSink, SValue, quirks::SValueToBoolQ};

impl SValue {
    pub fn q_and<Q>(&self, other: &SValue, sink: &mut Q) -> bool
    where
        Q: QuirkSink<SValueToBoolQ>,
    {
        let a = self.q_as_bool(sink);
        let b = other.q_as_bool(sink);
        a && b
    }
    pub fn q_or<Q>(&self, other: &SValue, sink: &mut Q) -> bool
    where
        Q: QuirkSink<SValueToBoolQ>,
    {
        let a = self.q_as_bool(sink);
        let b = other.q_as_bool(sink);
        a || b
    }
    pub fn q_not<Q>(&self, sink: &mut Q) -> bool
    where
        Q: QuirkSink<SValueToBoolQ>,
    {
        !self.q_as_bool(sink)
    }

    pub(crate) fn and(&self, other: &SValue) -> bool {
        // there should never be any errors for user programs as expression
        // blocks (texts/numbers) can't be placed in condition wholes of shape <..>.
        // Because of that errors can be ignored
        self.q_and(other, &mut ())
    }

    pub(crate) fn or(&self, other: &SValue) -> bool {
        // there should never be any errors for user programs as expression
        // blocks (texts/numbers) can't be placed in condition wholes of shape <..>.
        // Because of that errors can be ignored
        self.q_or(other, &mut ())
    }

    pub(crate) fn not(&self) -> bool {
        // there should never be any errors for user programs as expression
        // blocks (texts/numbers) can't be placed in condition wholes of shape <..>.
        // Because of that errors can be ignored
        self.q_not(&mut ())
    }
}
