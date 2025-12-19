use crate::{SNumber, quirks::SNumberToFloatQ};

mod round;
mod sqrt;
mod trig;

macro_rules! lift_snumber_method {
    ($([$($qs: ty),*])? $name: ident (pass)) => {
        impl $crate::SValue {
            pub fn $name<Q>(&self, sink: &mut Q) -> $crate::SNumber
                where Q: $crate::QuirkSink<$crate::quirks::SValueToNumberQ>, $($(Q: $crate::QuirkSink<$qs>),+)?
            {
                self.q_as_number_strict_bool_text(sink).$name(sink)
            }
        }
    };
    ($([$($qs: ty),*])? $name: ident) => {
        impl $crate::SValue {
            pub fn $name<Q>(&self, sink: &mut Q) -> $crate::SNumber
                where Q: $crate::QuirkSink<$crate::quirks::SValueToNumberQ>, $($(Q: $crate::QuirkSink<$qs>),+)?
            {
                self.q_as_number_strict_bool_text(sink).$name()
            }
        }
    };
}

impl SNumber {
    pub fn abs(&self) -> SNumber {
        match self {
            // TODO add warning for i64::MIN
            Self::Int(i) => Self::Int(i.saturating_abs()),
            Self::Float(f) => Self::Float(f.abs()),
        }
    }
}
impl crate::SValue {
    pub fn q_abs<Q>(&self, sink: &mut Q) -> SNumber
    where
        Q: crate::QuirkSink<crate::quirks::SValueToNumberQ>,
    {
        self.q_as_number_strict_bool_text(sink).abs()
    }
}

lift_snumber_method! {floor}
lift_snumber_method! {round}
lift_snumber_method! {ceil}
lift_snumber_method! {sqrt}

lift_snumber_method! {[SNumberToFloatQ] q_cos (pass)}
lift_snumber_method! {[SNumberToFloatQ] q_sin (pass)}
lift_snumber_method! {[SNumberToFloatQ] q_tan (pass)}

lift_snumber_method! {[SNumberToFloatQ] q_acos (pass)}
lift_snumber_method! {[SNumberToFloatQ] q_asin (pass)}
lift_snumber_method! {[SNumberToFloatQ] q_atan (pass)}

/*
lift_snumber_method! {q_ln}
lift_snumber_method! {q_log10}
lift_snumber_method! {q_exp}
lift_snumber_method! {q_pow10}
*/
