use crate::{SNumber, SValue};

macro_rules! test_svalue_prim {
    ($svalue: expr, $inner: expr) => {
        let src = $svalue;

        let j = serde_json::to_value(&src).unwrap();
        assert_eq!(serde_json::json!($inner), j);

        let deser: SValue = serde_json::from_value(j).unwrap();

        assert_eq!(src, deser);
    };
}

macro_rules! test_snum_prim {
    ($svalue: expr, $inner: expr) => {
        let src = $svalue;

        let j = serde_json::to_value(&src).unwrap();
        assert_eq!(serde_json::json!($inner), j);

        let deser: SNumber = serde_json::from_value(j).unwrap();

        assert_eq!(src, deser);
    };
}

#[test]
fn test_serde_num_i64() {
    test_snum_prim!(SNumber::Int(42), 42);
}
#[test]
fn test_serde_num_f64_no_dot() {
    test_snum_prim!(SNumber::Float(42.0), 42.0);
}

#[test]
fn test_serde_num_f64_with_dot() {
    test_snum_prim!(SNumber::Float(42.5), 42.5);
}

#[test]
fn test_serde_i64() {
    test_svalue_prim!(SValue::Int(42), 42);
}
#[test]
fn test_serde_f64_no_dot() {
    test_svalue_prim!(SValue::Float(42.0), 42.0);
}

#[test]
fn test_serde_f64_with_dot() {
    test_svalue_prim!(SValue::Float(42.5), 42.5);
}

#[test]
fn test_serde_bool() {
    test_svalue_prim!(SValue::Bool(true), true);
}

#[test]
fn test_serde_text() {
    test_svalue_prim!(SValue::Text("abc".into()), "abc");
}
