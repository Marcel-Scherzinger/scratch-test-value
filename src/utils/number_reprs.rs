use crate::SValue;
use itertools::Itertools;

/// This method is used to perform checks/tests for all ways an integer
/// number could be provided as a Scratch value.
///
/// - Directly as an integer ([`SValue::Int`])
/// - As a float ending with `.0` ([`SValue::Float`])
/// - As a textual representation of the digits (without `.0`), ([`SValue::Text`])
/// - As a textual representation of the digits with trailing `.0`, ([`SValue::Text`])
///
/// Those four representations are returned as a [`Vec<SValue>`]
pub(crate) fn int2reprs(a: i64) -> Vec<SValue> {
    vec![
        SValue::Int(a),
        SValue::Float(a as f64),
        SValue::Text(a.to_string().into()),
        SValue::Text((a as f64).to_string().into()),
    ]
}

/// Like [int2reprs] but if the parameter is 0 this also returns
/// the negative floating point version of 0.
#[allow(non_snake_case)]
pub(crate) fn int2reprsZ(a: i64) -> Vec<SValue> {
    if a == 0 {
        vec![
            SValue::Int(0),
            SValue::Float(0.0),
            SValue::Float(-0.0),
            SValue::Text("0".into()),
            SValue::Text("0.0".into()),
            SValue::Text("-0.0".into()),
        ]
    } else {
        vec![
            SValue::Int(a),
            SValue::Float(a as f64),
            SValue::Text(a.to_string().into()),
            SValue::Text((a as f64).to_string().into()),
        ]
    }
}

/// This method returns all variations these two numbers could be
/// represented as Scratch values.
///
/// It's the cartesian product of [`int2reprs`]
///
/// See also [ints2reprsZ]
pub(crate) fn ints2reprs(a: i64, b: i64) -> impl Iterator<Item = (SValue, SValue)> {
    int2reprs(a).into_iter().cartesian_product(int2reprs(b))
}
/// This method returns all variations these two numbers could be
/// represented as Scratch values. 0 will also returned as -0.0
///
/// It's the cartesian product of [`int2reprs`]
///
/// See also [ints2reprs]
#[allow(non_snake_case)]
pub(crate) fn ints2reprsZ(a: i64, b: i64) -> impl Iterator<Item = (SValue, SValue)> {
    int2reprsZ(a).into_iter().cartesian_product(int2reprs(b))
}

#[cfg(test)]
mod tests {
    use super::{int2reprs, int2reprsZ};
    use crate::SValue;

    #[test]
    fn test_positive() {
        assert_eq!(
            vec![
                SValue::Int(2),
                SValue::Float(2.0),
                SValue::Text("2".into()),
                SValue::Text("2.0".into())
            ],
            int2reprs(2)
        );
        assert_eq!(
            vec![
                SValue::Int(2),
                SValue::Float(2.0),
                SValue::Text("2".into()),
                SValue::Text("2.0".into())
            ],
            int2reprsZ(2)
        );
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            vec![
                SValue::Int(-2),
                SValue::Float(-2.0),
                SValue::Text("-2".into()),
                SValue::Text("-2.0".into())
            ],
            int2reprs(-2)
        );
        assert_eq!(
            vec![
                SValue::Int(-2),
                SValue::Float(-2.0),
                SValue::Text("-2".into()),
                SValue::Text("-2.0".into())
            ],
            int2reprsZ(-2)
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            vec![
                SValue::Int(0),
                SValue::Float(0.0),
                SValue::Text("0".into()),
                SValue::Text("0".into())
            ],
            int2reprs(0)
        );
        assert_eq!(
            vec![
                SValue::Int(0),
                SValue::Float(0.0),
                SValue::Float(-0.0),
                SValue::Text("0".into()),
                SValue::Text("0.0".into()),
                SValue::Text("-0.0".into())
            ],
            int2reprsZ(0)
        );
    }
}
