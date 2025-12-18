use crate::SValue;

#[test]
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn test_empty_string() {
    assert!(SValue::from("") < SValue::from("a"));
    assert!(SValue::from("") < SValue::INFINITY);
    assert!(SValue::from("") < SValue::NEG_INFINITY);
    assert!(SValue::from("") < SValue::POS_ZERO);
    assert!(SValue::from("") < SValue::NEG_ZERO);
    // swapped
    assert!(!(SValue::from("a") < SValue::from("")));
    assert!(!(SValue::INFINITY < SValue::from("")));
    assert!(!(SValue::NEG_INFINITY < SValue::from("")));
    assert!(!(SValue::POS_ZERO < SValue::from("")));
    assert!(!(SValue::NEG_ZERO < SValue::from("")));
}

#[test]
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn test_bool_ordering() {
    assert!(SValue::Bool(true) < SValue::Text("2".into()));
    assert!(SValue::Bool(false) < SValue::Text("2".into()));

    assert!(!(SValue::Text("true".into()) < SValue::Text("2".into())));
    assert!(!(SValue::Text("false".into()) < SValue::Text("2".into())));
}
