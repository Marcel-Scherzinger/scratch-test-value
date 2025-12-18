use crate::{
    SValue,
    utils::{int2reprs, int2reprsZ, ints2reprsZ},
};

#[test]
fn test_add() {
    assert_eq!(SValue::Int(0), SValue::NAN + SValue::NAN);
    assert_eq!(SValue::Int(-2), SValue::NAN + SValue::Int(-2));
    assert_eq!(SValue::Int(-2), SValue::Int(-2) + SValue::NAN);
    assert_eq!(SValue::Int(2), SValue::NAN + SValue::Int(2));
    assert_eq!(SValue::Int(2), SValue::Int(2) + SValue::NAN);
    assert_eq!(SValue::INFINITY, SValue::INFINITY + SValue::NAN);
    assert_eq!(SValue::NEG_INFINITY, SValue::NEG_INFINITY + SValue::NAN);
    assert_eq!(SValue::Float(4.2), SValue::Float(4.2) + SValue::NAN);
    assert_eq!(SValue::Float(4.2), SValue::NAN + SValue::Float(4.2));

    assert_eq!(SValue::NEG_INFINITY, SValue::NAN + SValue::NEG_INFINITY);
    assert_eq!(SValue::INFINITY, SValue::NAN + SValue::INFINITY);

    for zero in int2reprsZ(0) {
        assert_eq!(SValue::POS_ZERO, SValue::NAN + zero.clone());
        assert_eq!(SValue::NEG_ZERO, SValue::NAN + zero.clone());
        assert_eq!(SValue::POS_ZERO, zero.clone() + SValue::NAN);
        assert_eq!(SValue::NEG_ZERO, zero.clone() + SValue::NAN);
        assert_eq!(SValue::INFINITY, SValue::INFINITY + zero.clone());
        assert_eq!(SValue::INFINITY, zero.clone() + SValue::INFINITY);
        assert_eq!(SValue::NEG_INFINITY, SValue::NEG_INFINITY + zero.clone());
        assert_eq!(SValue::NEG_INFINITY, zero + SValue::NEG_INFINITY);
    }
    for (zero1, zero2) in ints2reprsZ(0, 0) {
        assert_eq!(SValue::Int(0), zero1 + zero2);
    }
    assert_eq!(SValue::Float(6.6), SValue::Float(4.5) + SValue::Float(2.1));

    assert_eq!(SValue::INFINITY, SValue::INFINITY + SValue::INFINITY);
    assert_eq!(SValue::NAN, SValue::INFINITY + SValue::NEG_INFINITY);
    assert_eq!(SValue::INFINITY, SValue::INFINITY + SValue::Float(4.2));
    assert_eq!(SValue::INFINITY, SValue::INFINITY + SValue::Float(-4.2));

    assert_eq!(SValue::INFINITY, SValue::Float(4.2) + SValue::INFINITY);
    assert_eq!(SValue::INFINITY, SValue::Float(-4.2) + SValue::INFINITY);
    assert_eq!(SValue::NAN, SValue::NEG_INFINITY + SValue::INFINITY);
    assert_eq!(
        SValue::NEG_INFINITY,
        SValue::NEG_INFINITY + SValue::NEG_INFINITY
    );

    for finite in [4, -4] {
        for num in int2reprsZ(finite) {
            assert_eq!(SValue::NEG_INFINITY, num.clone() + SValue::NEG_INFINITY);
            assert_eq!(SValue::NEG_INFINITY, SValue::NEG_INFINITY + num);
        }
    }
    for v in int2reprs(4) {
        assert_eq!(SValue::Float(4.0), SValue::POS_ZERO + v.clone());
        assert_eq!(SValue::Float(4.0), SValue::NEG_ZERO + v.clone());

        assert_eq!(SValue::Float(4.0), v.clone() + SValue::POS_ZERO);
        assert_eq!(SValue::Float(4.0), v + SValue::NEG_ZERO);
    }
    for v in int2reprs(-4) {
        assert_eq!(SValue::Float(-4.0), SValue::POS_ZERO + v.clone());
        assert_eq!(SValue::Float(-4.0), SValue::NEG_ZERO + v.clone());
        assert_eq!(SValue::Float(-4.0), v.clone() + SValue::POS_ZERO);
        assert_eq!(SValue::Float(-4.0), v + SValue::NEG_ZERO);
    }

    assert_eq!(SValue::Int(14), SValue::Int(12) + SValue::Int(2));
    assert_eq!(SValue::Int(14), SValue::Int(12) + SValue::Text("2".into()));
    assert_eq!(SValue::Float(6.6), SValue::Float(4.5) + SValue::Float(2.1));
    assert_eq!(SValue::Int(10), SValue::Int(10) + SValue::Text("a".into()));
    assert_eq!(SValue::Int(10), SValue::Int(10) + SValue::Text("".into()));
}

#[test]
fn test_float_add_error() {
    assert_eq!(
        SValue::Float(6.300000000000001),
        SValue::Float(4.2) + SValue::Float(2.1)
    );
}
