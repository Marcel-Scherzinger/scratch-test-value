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

impl PartialOrd for SValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.q_lt(other, &mut ()) {
            Some(std::cmp::Ordering::Less)
        } else if self.q_gt(other, &mut ()) {
            Some(std::cmp::Ordering::Greater)
        } else {
            (self == other).then_some(std::cmp::Ordering::Equal)
        }
    }
}

impl PartialOrd for SNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.q_lt(other, &mut ()) {
            Some(std::cmp::Ordering::Less)
        } else if self.q_gt(other, &mut ()) {
            Some(std::cmp::Ordering::Greater)
        } else {
            (self == other).then_some(std::cmp::Ordering::Equal)
        }
    }
}
