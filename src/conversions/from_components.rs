#[cfg(feature = "serde_json")]
use crate::SNumber;
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

#[cfg(feature = "serde_json")]
impl From<serde_json::Number> for SValue {
    fn from(value: serde_json::Number) -> SValue {
        SNumber::from(value).into()
    }
}
#[cfg(feature = "serde_json")]
impl From<serde_json::Number> for SNumber {
    fn from(value: serde_json::Number) -> SNumber {
        if let Some(n) = value.as_f64() {
            Self::Float(n)
        } else if let Some(n) = value.as_i64() {
            Self::Int(n)
        } else if let Some(n) = value.as_u64() {
            if let Ok(i) = n.try_into() {
                Self::Int(i)
            } else {
                Self::Float(n as f64)
            }
        } else {
            Self::Int(0)
        }
    }
}

#[cfg(feature = "serde_json")]
#[derive(Debug, PartialEq, Eq, Clone, thiserror::Error)]
#[error("json value can't be converted into Scratch value (object/array?): {0}")]
pub struct JsonValueNoSValue(pub serde_json::Value);

#[cfg(feature = "serde_json")]
impl TryFrom<serde_json::Value> for SValue {
    type Error = JsonValueNoSValue;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let initial = if let Some(num) = value.as_number() {
            // TODO: HINT integer out of range case is ignored
            SValue::from(num.clone())
        } else if let Some(boo) = value.as_bool() {
            SValue::Bool(boo)
        } else if let Some(tex) = value.as_str() {
            SValue::Text(tex.into())
        } else {
            return Err(JsonValueNoSValue(value));
        };
        Ok(initial)
    }
}
