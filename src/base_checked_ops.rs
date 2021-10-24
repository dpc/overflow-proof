use super::Checked;

pub trait CheckedAdd<Rhs = Self> {
    type Output;

    fn checked_add(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedSub<Rhs = Self> {
    type Output;

    fn checked_sub(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedMul<Rhs = Self> {
    type Output;

    fn checked_mul(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedDiv<Rhs = Self> {
    type Output;

    fn checked_div(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedRem<Rhs = Self> {
    type Output;

    fn checked_rem(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedAbs : Sized {
    type Output;

    fn checked_abs(self) -> Option<Self>;
}

pub trait CheckedNeg : Sized {
    type Output;

    fn checked_neg(self) -> Option<Self>;
}


macro_rules! impl_checked_trait_2_for {
    ($checked_t:tt, $checked_op:ident, $t:ty) => {

        impl $checked_t for $t {
            type Output = $t;

            fn $checked_op(self, rhs: Self) -> Option<Self::Output> {
                self.$checked_op(rhs)
            }
        }

        impl<D> $checked_t<Checked<$t, D>> for $t {
            type Output = $t;

            fn $checked_op(self, rhs: Checked<$t, D>) -> Option<Self::Output> {
                self.$checked_op(rhs.v)
            }
        }
    }
}

macro_rules! impl_checked_trait_1_for {
    ($checked_t:ty, $checked_op:ident, $t:ty) => {

        impl $checked_t for $t {
            type Output = $t;

            fn $checked_op(self) -> Option<Self::Output> {
                self.$checked_op()
            }
        }
    }
}

macro_rules! impl_checked_all {
    ($t:ty) => {
        impl_checked_trait_2_for!(CheckedAdd, checked_add, $t);
        impl_checked_trait_2_for!(CheckedSub, checked_sub, $t);
        impl_checked_trait_2_for!(CheckedMul, checked_mul, $t);
        impl_checked_trait_2_for!(CheckedDiv, checked_div, $t);
        impl_checked_trait_2_for!(CheckedRem, checked_rem, $t);
        impl_checked_trait_1_for!(CheckedNeg, checked_neg, $t);
    }
}

impl_checked_all!(usize);
impl_checked_all!(isize);
impl_checked_all!(u8);
impl_checked_all!(i8);
impl_checked_all!(u16);
impl_checked_all!(i16);
impl_checked_all!(u32);
impl_checked_all!(i32);
impl_checked_all!(u64);
impl_checked_all!(i64);
impl_checked_all!(u128);
impl_checked_all!(i128);

// nightly only
/*
impl_checked_trait_1_for!(CheckedAbs, checked_abs, std::num::NonZeroIsize);
impl_checked_trait_1_for!(CheckedAbs, checked_abs, std::num::NonZeroI8);
impl_checked_trait_1_for!(CheckedAbs, checked_abs, std::num::NonZeroI16);
impl_checked_trait_1_for!(CheckedAbs, checked_abs, std::num::NonZeroI32);
impl_checked_trait_1_for!(CheckedAbs, checked_abs, std::num::NonZeroI64);
impl_checked_trait_1_for!(CheckedAbs, checked_abs, std::num::NonZeroI128);
*/

