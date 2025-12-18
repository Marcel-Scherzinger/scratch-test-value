use crate::SValue;
use crate::utils::int2reprsZ;
fn sv(s: impl Into<SValue>) -> SValue {
    s.into()
}

fn test_both(tasks: impl IntoIterator<Item = ((SValue, SValue), SValue)>) {
    for ((a, b), expected_prod) in tasks {
        assert_eq!(
            expected_prod,
            &a * &b,
            "multiplication of {a} and {b}, expected {expected_prod}"
        );
        assert_eq!(
            expected_prod,
            &b * &a,
            "multiplication of {b} and {a}, expected {expected_prod}"
        );
    }
}

#[test]
fn test_mul_zero() {
    for zero1 in int2reprsZ(0) {
        for zero2 in int2reprsZ(0) {
            test_both(vec![((zero1.clone(), zero2), sv(0))]);
        }
        test_both(vec![
            ((SValue::NAN, zero1.clone()), sv(0)),
            ((SValue::INFINITY, zero1.clone()), SValue::NAN),
            ((SValue::NEG_INFINITY, zero1), SValue::NAN),
        ]);
    }
}

#[test]
fn test_mul_finite() {
    for number in 1..100 {
        for x in int2reprsZ(number) {
            test_both(vec![((x.clone(), SValue::Int(2)), SValue::Int(number * 2))]);
            test_both(vec![(
                (x.clone(), SValue::Float(2.0)),
                SValue::Float(number as f64 * 2.0),
            )]);
            test_both(vec![((x, SValue::Int(0)), SValue::Int(0))]);
        }
    }
}
#[test]
fn test_mul_special() {
    test_both(vec![
        ((SValue::NAN, SValue::INFINITY), SValue::NAN),
        ((SValue::NAN, SValue::NEG_INFINITY), SValue::NAN),
        (
            (SValue::INFINITY, SValue::NEG_INFINITY),
            SValue::NEG_INFINITY,
        ),
        ((SValue::INFINITY, SValue::INFINITY), SValue::INFINITY),
        (
            (SValue::NEG_INFINITY, SValue::NEG_INFINITY),
            SValue::INFINITY,
        ),
    ]);
}
