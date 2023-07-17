// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign, Sub, SubAssign,
};

use crate::seal::Seal;

pub trait IntSeal:
    Seal
    + Not
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + MulAssign
    + Div
    + DivAssign
    + Shr<u8>
    + ShrAssign<u8>
    + Shl<u8>
    + ShlAssign<u8>
    + Eq
    + PartialEq
    + Sized
{
}

impl<T> IntSeal for T where
    T: Seal
        + Not
        + Add
        + AddAssign
        + Sub
        + SubAssign
        + Mul
        + MulAssign
        + Div
        + DivAssign
        + Shr<u8>
        + ShrAssign<u8>
        + Shl<u8>
        + ShlAssign<u8>
        + Eq
        + PartialEq
        + Sized
{
}

/// Trait implemented for integer primitives
pub trait Int: IntSeal {}

impl Int for u8 {}
impl Int for i8 {}
impl Int for u16 {}
impl Int for i16 {}
impl Int for u32 {}
impl Int for i32 {}
impl Int for u64 {}
impl Int for i64 {}
impl Int for u128 {}
impl Int for i128 {}

pub trait UIntSeal: Seal + Int + From<u8> + Copy + Clone {}

impl<T> UIntSeal for T where T: Seal + Int + From<u8> + Copy + Clone {}

/// Trait implemented for unsigned integer primitives
pub trait UInt: UIntSeal {
    /// The minimum value of an unsigned integer, 0
    const ZERO: Self;
    /// Size of the primitive, in bits
    const BITS: u8;

    /// Grab the little byte.
    fn little(&self) -> u8;
}

impl UInt for u8 {
    const BITS: u8 = 8;
    const ZERO: u8 = u8::MIN;

    fn little(&self) -> u8 {
        *self
    }
}

impl UInt for u16 {
    const BITS: u8 = 16;
    const ZERO: u16 = u16::MIN;

    fn little(&self) -> u8 {
        let [byte, _] = self.to_le_bytes();

        byte
    }
}

impl UInt for u32 {
    const BITS: u8 = 32;
    const ZERO: u32 = u32::MIN;

    fn little(&self) -> u8 {
        let [byte, _, _, _] = self.to_le_bytes();

        byte
    }
}

impl UInt for u64 {
    const BITS: u8 = 64;
    const ZERO: u64 = u64::MIN;

    fn little(&self) -> u8 {
        let [byte, _, _, _, _, _, _, _] = self.to_le_bytes();

        byte
    }
}

impl UInt for u128 {
    const BITS: u8 = 128;
    const ZERO: u128 = u128::MIN;

    fn little(&self) -> u8 {
        let [byte, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] =
            self.to_le_bytes();

        byte
    }
}
