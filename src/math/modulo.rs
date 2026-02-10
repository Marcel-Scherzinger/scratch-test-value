use std::ops::Rem;

use crate::{QuirkSink, SNumber, quirks::SNumberToFloatQ};

impl SNumber {
    /// This mimics the JS remainder operator <https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Remainder>
    pub fn q_modulo<Q>(&self, other: &SNumber, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        let dividend = self.q_as_float_nan_is_zero(sink);
        let divisor = other.q_as_float_nan_is_zero(sink);
        let remainder = dividend.rem(divisor);

        let diff_sign = (dividend < 0.0 && divisor > 0.0) || (dividend > 0.0 && divisor < 0.0);
        SNumber::Float(
            if diff_sign && (divisor != 1.0 && divisor != -1.0 && divisor.is_finite()) {
                remainder + divisor
            } else {
                remainder
            },
        )
    }
    #[allow(unused)]
    pub(crate) fn modulo(&self, other: &SNumber) -> SNumber {
        self.q_modulo(other, &mut ())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Sub;

    use crate::SNumber;
    const ZEROS: [SNumber; 3] = [SNumber::POS_ZERO, SNumber::NEG_ZERO, SNumber::Int(0)];

    #[test]
    fn test_normal() {
        assert_eq!(SNumber::Int(1), SNumber::Int(101).modulo(&SNumber::Int(2)));
    }

    #[test]
    fn test_floating() {
        assert_eq!(
            SNumber::Float(2.5),
            SNumber::Float(7.5).modulo(&SNumber::Int(5))
        );
        assert_eq!(
            SNumber::Int(2),
            SNumber::Float(7.5).modulo(&SNumber::Float(5.5))
        );

        assert!(
            SNumber::Float(12.25)
                .modulo(&SNumber::Float(1.45))
                .q_as_float(&mut ())
                .sub(0.65)
                .abs()
                < 0.0001
        );
        assert!(
            SNumber::Float(21.3)
                .modulo(&SNumber::Float(0.7))
                .q_as_float(&mut ())
                .sub(0.3)
                .abs()
                < 0.0001
        );
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            SNumber::Int(4),
            SNumber::Int(-1001).modulo(&SNumber::Int(5))
        );
        assert_eq!(
            SNumber::Int(-1),
            SNumber::Int(101).modulo(&SNumber::Int(-2))
        );
        assert_eq!(
            SNumber::Int(-4),
            SNumber::Int(1001).modulo(&SNumber::Int(-5))
        );
        assert_eq!(SNumber::Int(1), SNumber::Int(1001).modulo(&SNumber::Int(5)));
        assert_eq!(
            SNumber::Int(-1),
            SNumber::Int(-1001).modulo(&SNumber::Int(-5))
        );
        assert_eq!(
            SNumber::Float(-3.5),
            SNumber::Float(7.5).modulo(&SNumber::Float(-5.5))
        );
    }

    #[test]
    fn test_mod_pm1() {
        for one in [-1, 1] {
            for i in [10, -10, 100, 0] {
                assert_eq!(SNumber::Int(0), SNumber::Int(i).modulo(&SNumber::Int(one)));
            }
            assert_eq!(SNumber::Int(0), SNumber::NAN.modulo(&SNumber::Int(one)));
            assert_eq!(SNumber::NAN, SNumber::INFINITY.modulo(&SNumber::Int(one)));
            assert_eq!(
                SNumber::NAN,
                SNumber::NEG_INFINITY.modulo(&SNumber::Int(one))
            );
        }
    }

    #[test]
    fn test_mod_special() {
        for zero1 in ZEROS {
            for zero2 in ZEROS {
                assert_eq!(SNumber::NAN, zero1.modulo(&zero2));
            }
            assert_eq!(SNumber::Int(0), zero1.modulo(&SNumber::INFINITY));
            assert_eq!(SNumber::Int(0), zero1.modulo(&SNumber::NEG_INFINITY));
            assert_eq!(SNumber::NAN, zero1.modulo(&SNumber::NAN));
        }
        for zero in ZEROS {
            assert_eq!(SNumber::NAN, SNumber::NAN.modulo(&zero));
            assert_eq!(SNumber::NAN, SNumber::INFINITY.modulo(&zero));
            assert_eq!(SNumber::NAN, SNumber::NEG_INFINITY.modulo(&zero));
            assert_eq!(SNumber::NAN, SNumber::Int(10).modulo(&zero));
            assert_eq!(SNumber::NAN, SNumber::Int(-10).modulo(&zero));
        }
        assert_eq!(
            SNumber::NAN,
            SNumber::NEG_INFINITY.modulo(&SNumber::NEG_INFINITY)
        );
        assert_eq!(
            SNumber::NAN,
            SNumber::NEG_INFINITY.modulo(&SNumber::INFINITY)
        );
        assert_eq!(SNumber::NAN, SNumber::INFINITY.modulo(&SNumber::INFINITY));
        assert_eq!(
            SNumber::NAN,
            SNumber::INFINITY.modulo(&SNumber::NEG_INFINITY)
        );
        assert_eq!(SNumber::NAN, SNumber::NAN.modulo(&SNumber::NAN));
    }

    #[test]
    fn test_number_mod_inf() {
        assert_eq!(
            SNumber::Int(-1001),
            SNumber::Int(-1001).modulo(&SNumber::NEG_INFINITY)
        );

        assert_eq!(
            SNumber::Int(-1001),
            SNumber::Int(-1001).modulo(&SNumber::INFINITY)
        );

        assert_eq!(
            SNumber::Int(1001),
            SNumber::Int(1001).modulo(&SNumber::INFINITY)
        );

        assert_eq!(
            SNumber::Int(1001),
            SNumber::Int(1001).modulo(&SNumber::NEG_INFINITY)
        );
    }
}
