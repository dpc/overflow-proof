#![doc = include_str!("../README.md")]

use std::cmp::{Eq, PartialEq};
use std::marker::PhantomData;
use std::ops::{Add, Deref, Div, Mul, Sub};

/// Base traits & ops for inner types wrapped by `Checked` and `Unchecked`
// Why weren't these made into stdlib traits from the ground up?! :(
mod base_checked_ops;
pub use base_checked_ops::*;

/// Marker for [`Checked`] that can be converted to the inner type semi-automatically.
#[derive(Debug)]
pub struct WithDeref;

/// Marker for [`Checked`] that must be explicitly converted to the inner type.
#[derive(Debug)]
pub struct WithoutDeref;

/// A wrapper around a numeric type, containing a valid value,
/// that will perform overflow checks on arithmetic operations.
///
/// Any arithmetic operation (like `Add`, `Sub`...) will be performed using checked
/// arithmetic and return a [`Unchecked`] type which will track overflow state. Additional
/// chaining additional arithmetic operations is possible witoutchecking overflow in intermediate steps.
/// Only when the final result is to be used, it's neccessary to call [`Unchecked::check()`]
/// to convert back to [`Checked`] value.
///
/// `T` is the inner type (`u8`, `i16`, etc.) wrapped by this type.
///
/// `D` is a marker controlling automatic conversion to inner type. It defaults to [`WithDeref`]
/// which results in semi-implicit conversion to `T` available (like `Deref`). For values where handling
/// overflow is particularily important and opting out of it could have serious consequences,
/// [`WithoutDeref`] can be used, which will require calling
/// an explicit conversion function to convert to the inner type.
#[derive(Debug)]
pub struct Checked<T, D = WithDeref> {
    v: T,
    _deref: PhantomData<D>,
}

impl<T, D> Clone for Checked<T, D>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            v: self.v.clone(),
            _deref: self._deref,
        }
    }
}

impl<T, D> Copy for Checked<T, D> where T: Copy {}

impl<T, D> From<T> for Checked<T, D> {
    fn from(v: T) -> Self {
        Self { v, _deref: PhantomData }
    }
}

impl<T, D> Checked<T, D> {
    pub fn into_inner(self) -> T {
        self.v
    }
}

impl<T, D> Checked<T, D>
where
    T: Clone,
{
    pub fn to_inner(&self) -> T {
        self.v.clone()
    }

}

impl<T> Checked<T, WithDeref> {
    pub fn new_with_deref(v: T) -> Checked<T, WithDeref> {
        Self {
            v,
            _deref: PhantomData,
        }
    }

    pub fn new(v: T) -> Self {
        Self {
            v,
            _deref: PhantomData,
        }
    }
}

impl<T> Checked<T, WithoutDeref> {
    pub fn new_without_deref(v: T) -> Checked<T, WithoutDeref> {
        Self {
            v,
            _deref: PhantomData,
        }
    }

}

impl<T> Deref for Checked<T, WithDeref> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}


/// Intermediate result of artimetic operations on [`Checked`] value that might contain overflow
///
/// At any point contains either a value of the inner type `T`,
/// or a marker that overflow was detected and any subsequent
/// artimetic operations will keep resulting
/// in overflow, similiarly to how NaN behaves.
#[derive(Debug)]
pub struct Unchecked<T, D = WithoutDeref> {
    v: Option<T>,
    _deref: PhantomData<D>,
}

impl<T, D> Clone for Unchecked<T, D>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            v: self.v.clone(),
            _deref: self._deref,
        }
    }
}

impl<T, D> Copy for Unchecked<T, D> where T: Copy {}

impl<T, D> Unchecked<T, D> {
    /// Convert back to [`Checked`].
    ///
    /// Returns `None` if inner value denotes overflow.
    pub fn check(self) -> Option<Checked<T, D>> {
        self.v.map(|v| Checked {
            v,
            _deref: PhantomData,
        })
    }
}

macro_rules! impl_op {
    ($op:tt,$checked_op:tt,$method:ident,$checked_method:ident) => {
        impl<T, D, Rhs> $op<Rhs> for Checked<T, D>
        where
            T: $checked_op<Rhs>,
        {
            type Output = Unchecked<<T as $checked_op<Rhs>>::Output, D>;

            fn $method(self, rhs: Rhs) -> Self::Output {
                Unchecked {
                    v: self.v.$checked_method(rhs),
                    _deref: self._deref,
                }
            }
        }

        impl<T, D, Rhs> $op<Rhs> for Unchecked<T, D>
        where
            T: $checked_op<Rhs>,
        {
            type Output = Unchecked<<T as $checked_op<Rhs>>::Output, D>;

            fn $method(self, rhs: Rhs) -> Self::Output {
                Unchecked {
                    v: self.v.and_then(|v| v.$checked_method(rhs)),
                    _deref: self._deref,
                }
            }
        }
    };
}

impl_op!(Add, CheckedAdd, add, checked_add);
impl_op!(Sub, CheckedSub, sub, checked_sub);
impl_op!(Mul, CheckedMul, mul, checked_mul);
impl_op!(Div, CheckedDiv, div, checked_div);

impl<T, D1, D2> PartialEq<Checked<T, D1>> for Checked<T, D2>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &Checked<T, D1>) -> bool {
        self.v.eq(&other.v)
    }
}
impl<T, D1> Eq for Checked<T, D1> where T: PartialEq<T> {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            *{ Checked::new_with_deref(1u8) * 12 - 2 }
                .check()
                .expect("no oveflow"),
            10
        );

        assert_eq!(
            { Checked::new(1u8) * 12 - 2 }.check().expect("no oveflow"),
            Checked::new(10)
        );

        assert!({ Checked::new(1u8) + u8::MAX }.check().is_none());
        assert!({ Checked::new(255u8) + 5 - 100 }.check().is_none());
    }
}
