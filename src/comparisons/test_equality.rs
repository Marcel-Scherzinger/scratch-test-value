use crate::{
    SNumber, SValue,
    utils::{int2reprs, int2reprsZ, ints2reprsZ},
};

#[test]
fn test_nan_cmp() {
    assert_eq!(SValue::NAN, SValue::NAN);
    assert_eq!(SNumber::NAN, SNumber::NAN);

    for val in vec![
        SValue::NEG_ZERO,
        SValue::POS_ZERO,
        SValue::INFINITY,
        SValue::NEG_INFINITY,
        SValue::Float(2.9),
        SValue::Float(-2.9),
        SValue::Text("Test".into()),
        SValue::Text("".into()),
        SValue::Bool(true),
        SValue::Bool(false),
        SValue::Int(1),
        SValue::Int(0),
        SValue::Int(-20),
    ] {
        assert_ne!(SValue::NAN, val);
        assert_ne!(val, SValue::NAN);
    }
}

#[test]
fn test_pos_infinity_cmp() {
    assert_eq!(SValue::INFINITY, SValue::INFINITY);
    assert_eq!(SNumber::INFINITY, SNumber::INFINITY);

    for val in vec![
        SValue::NEG_ZERO,
        SValue::POS_ZERO,
        SValue::NEG_INFINITY,
        SValue::Float(2.9),
        SValue::Float(-2.9),
        SValue::Text("Test".into()),
        SValue::Text("".into()),
        SValue::Bool(true),
        SValue::Bool(false),
        SValue::Int(1),
        SValue::Int(0),
        SValue::Int(-20),
    ] {
        assert_ne!(SValue::INFINITY, val);
        assert_ne!(val, SValue::INFINITY)
    }
}

#[test]
fn test_neg_infinity_cmp() {
    assert_eq!(SValue::NEG_INFINITY, SValue::NEG_INFINITY);
    assert_eq!(SNumber::NEG_INFINITY, SNumber::NEG_INFINITY);

    for val in vec![
        SValue::NEG_ZERO,
        SValue::POS_ZERO,
        SValue::INFINITY,
        SValue::Float(2.9),
        SValue::Float(-2.9),
        SValue::Text("Test".into()),
        SValue::Text("".into()),
        SValue::Bool(true),
        SValue::Bool(false),
        SValue::Int(1),
        SValue::Int(0),
        SValue::Int(-20),
    ] {
        assert_ne!(SValue::NEG_INFINITY, val);
        assert_ne!(val, SValue::NEG_INFINITY)
    }
}

#[test]
fn test_zero_cmp() {
    for val in vec![
        SValue::NEG_INFINITY,
        SValue::INFINITY,
        SValue::Float(2.9),
        SValue::Float(-2.9),
        SValue::Text("Test".into()),
        SValue::Text("".into()),
        SValue::Bool(true),
        SValue::Int(1),
        SValue::Int(-20),
    ] {
        assert_ne!(SValue::POS_ZERO, val);
        assert_ne!(val, SValue::POS_ZERO);
        assert_ne!(SValue::NEG_ZERO, val);
        assert_ne!(val, SValue::NEG_ZERO);
        assert_ne!(val, SValue::Int(0));
        assert_ne!(SValue::Int(0), val);
    }
    for (zero1, zero2) in ints2reprsZ(0, 0) {
        assert_eq!(zero1, zero2);
    }
}

#[test]
fn test_numeric() {
    assert_eq!(SValue::Bool(true), SValue::Text("1".into()));
    assert_eq!(SValue::Bool(false), SValue::Text("0".into()));

    for zero in int2reprsZ(0) {
        assert_ne!(SValue::Bool(true), zero);
        assert_eq!(SValue::Bool(false), zero);
    }
    for one in int2reprs(1) {
        assert_eq!(SValue::Bool(true), one);
        assert_ne!(SValue::Bool(false), one);
    }
}

#[test]
fn test_textual_cmp() {
    assert_eq!(SValue::from("12"), SValue::Int(12));
    assert_eq!(SValue::from("12"), SValue::Float(12.0));
    assert_eq!(SValue::from("12.0"), SValue::Int(12));
    assert_eq!(SValue::from("12.0"), SValue::Float(12.0));

    assert_eq!(SValue::from("true"), SValue::Bool(true));
    assert_ne!(SValue::from("true"), SValue::Bool(false));
}

#[test]
fn test_bool_equality() {
    assert!(SValue::Bool(true) == SValue::Text("true".into()));
    assert!(SValue::Bool(false) == SValue::Text("false".into()));

    assert!(SValue::Bool(true) == SValue::Int(1));
    assert!(SValue::Bool(false) == SValue::Int(0));

    assert!(SValue::Int(1) != SValue::Text("true".into()));
    assert!(SValue::Int(0) != SValue::Text("false".into()));

    assert!(SValue::Text("true".into()) == SValue::Bool(true));
    assert!(SValue::Text("false".into()) == SValue::Bool(false));
}
