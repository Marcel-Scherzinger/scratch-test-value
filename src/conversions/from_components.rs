use crate::{ARc, SValue};

impl std::str::FromStr for SValue {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(int) = s.parse() {
            Self::Int(int)
        } else if let Ok(float) = s.parse() {
            if s.contains(".") || float < (i64::MIN as f64) || (i64::MAX as f64) < float {
                Self::Float(float)
            } else if let Ok(int) = s.parse() {
                Self::Int(int)
            } else {
                Self::Text(s.into())
            }
        } else if s == "true" {
            Self::Bool(true)
        } else if s == "false" {
            Self::Bool(false)
        } else {
            Self::Text(s.into())
        })
    }
}

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
