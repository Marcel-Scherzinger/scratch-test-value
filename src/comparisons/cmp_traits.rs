use crate::{SNumber, SValue};

impl PartialEq for SNumber {
    fn eq(&self, other: &Self) -> bool {
        self.q_eq(other, &mut ())
    }
}

impl PartialEq for SValue {
    fn eq(&self, other: &Self) -> bool {
        self.q_eq(other, &mut ())
    }
}

impl PartialEq<SValue> for &SValue {
    fn eq(&self, other: &SValue) -> bool {
        self.q_eq(other, &mut ())
    }
}

impl PartialEq<&SValue> for SValue {
    fn eq(&self, other: &&SValue) -> bool {
        self.q_eq(other, &mut ())
    }
}
