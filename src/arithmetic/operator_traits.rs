use std::ops::{Add, Div, Mul, Sub};

use crate::{SNumber, SValue};

macro_rules! impl_operator {
    ($trait: ident::$meth: ident, $my: ident) => {
        impl $trait<SValue> for SValue {
            type Output = SValue;
            fn $meth(self, rhs: Self) -> Self::Output {
                self.$my(&rhs, &mut ())
            }
        }
        impl $trait<&SValue> for SValue {
            type Output = SValue;
            fn $meth(self, rhs: &Self) -> Self::Output {
                self.$my(&rhs, &mut ())
            }
        }
        impl<'a> $trait<&'a SValue> for &'a SValue {
            type Output = SValue;
            fn $meth(self, rhs: Self) -> Self::Output {
                self.$my(&rhs, &mut ())
            }
        }
        impl $trait<SValue> for &SValue {
            type Output = SValue;
            fn $meth(self, rhs: SValue) -> Self::Output {
                self.$my(&rhs, &mut ())
            }
        }

        impl $trait<SNumber> for SNumber {
            type Output = SNumber;
            fn $meth(self, rhs: Self) -> Self::Output {
                self.$my(&rhs, &mut ())
            }
        }
        impl $trait<&SNumber> for SNumber {
            type Output = SNumber;
            fn $meth(self, rhs: &Self) -> Self::Output {
                self.$my(&rhs, &mut ())
            }
        }
        impl<'a> $trait<&'a SNumber> for &'a SNumber {
            type Output = SNumber;
            fn $meth(self, rhs: Self) -> Self::Output {
                self.$my(&rhs, &mut ())
            }
        }
        impl $trait<SNumber> for &SNumber {
            type Output = SNumber;
            fn $meth(self, rhs: SNumber) -> Self::Output {
                self.$my(&rhs, &mut ())
            }
        }
    };
}

impl_operator! {Div::div, q_div_numbers}
impl_operator! {Sub::sub, q_sub_numbers}
impl_operator! {Add::add, q_add_numbers}
impl_operator! {Mul::mul, q_mul_numbers}
