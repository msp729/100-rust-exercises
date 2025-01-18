// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
use core::ops::Add;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SaturatingU16(u16);

impl From<u16> for SaturatingU16 {
    fn from(x: u16) -> Self {
        Self(x)
    }
}
impl From<&u16> for SaturatingU16 {
    fn from(x: &u16) -> Self {
        Self(*x)
    }
}
impl From<u8> for SaturatingU16 {
    fn from(x: u8) -> Self {
        Self(x as u16)
    }
}
impl From<&u8> for SaturatingU16 {
    fn from(x: &u8) -> Self {
        Self(*x as u16)
    }
}
impl From<&Self> for SaturatingU16 {
    fn from(x: &Self) -> Self {
        *x
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl<T: Into<Self>> Add<T> for SaturatingU16 {
    type Output = Self;
    fn add(self, other: T) -> Self {
        Self(self.0.saturating_add(other.into().0))
    }
}
