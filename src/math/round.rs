use crate::SNumber;

impl SNumber {
    pub fn floor(&self) -> SNumber {
        match self {
            Self::Int(i) => Self::Int(*i),
            Self::Float(f) => {
                if f.is_nan() {
                    return Self::Int(0);
                }
                let f = f.floor();
                if f.abs() < (i64::MAX as f64) {
                    Self::Int(f as i64)
                } else {
                    Self::Float(f)
                }
            }
        }
    }
    pub fn ceil(&self) -> SNumber {
        match self {
            Self::Int(i) => Self::Int(*i),
            Self::Float(f) => {
                if f.is_nan() {
                    return Self::Int(0);
                }
                let f = f.ceil();
                if f.abs() < (i64::MAX as f64) {
                    Self::Int(f as i64)
                } else {
                    Self::Float(f)
                }
            }
        }
    }
    pub fn round(&self) -> SNumber {
        match self {
            Self::Int(i) => Self::Int(*i),
            Self::Float(f) => {
                if f.is_nan() {
                    return Self::Int(0);
                }
                let f = f.round();
                if f.abs() < (i64::MAX as f64) {
                    Self::Int(f as i64)
                } else {
                    Self::Float(f)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SNumber;

    #[test]
    fn test_round_negative() {
        assert_eq!(SNumber::Int(-2), SNumber::Float(-2.4).round());
    }

    #[test]
    fn test_round_special() {
        for s in [SNumber::INFINITY, SNumber::NEG_INFINITY] {
            assert_eq!(s, s.round());
            assert_eq!(s, s.ceil());
            assert_eq!(s, s.floor());
        }
        assert_eq!(SNumber::Int(0), SNumber::NAN.round());
        assert_eq!(SNumber::Int(0), SNumber::NAN.ceil());
        assert_eq!(SNumber::Int(0), SNumber::NAN.floor());
    }
}
