use crate::{ARc, SValue};

impl From<String> for SValue {
    fn from(value: String) -> Self {
        Self::Text(value.into())
    }
}

/// This implementation is currently only used in tests
/// so it is also annotated with `#[cfg(test)]`.
///
/// Maybe this will be added in normal code without the feature as well
#[cfg(test)]
impl<'a> From<&'a str> for SValue {
    fn from(value: &'a str) -> Self {
        Self::Text(value.into())
    }
}

impl From<ARc<str>> for SValue {
    fn from(value: ARc<str>) -> Self {
        Self::Text(value)
    }
}
impl From<i64> for SValue {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
impl From<f64> for SValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}
