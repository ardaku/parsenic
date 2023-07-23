// Copyright © 2022-2023 The Nucleide Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor,
    BitXorAssign, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use traitful::seal;

/// Trait implemented for integer primitives
#[seal(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128)]
pub trait Int:
    Add
    + AddAssign
    + BitAnd
    + BitAnd
    + BitAndAssign
    + BitOr
    + BitOrAssign
    + BitXor
    + BitXorAssign
    + Div
    + DivAssign
    + Mul
    + MulAssign
    + Not
    + Rem
    + RemAssign
    + Shl<u8>
    + ShlAssign<u8>
    + Shr<u8>
    + ShrAssign<u8>
    + Sub
    + SubAssign
    + Eq
    + PartialEq
    + Sized
    + 'static
{
}

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

/// Trait implemented for unsigned integer primitives
#[seal(u8, u16, u32, u64, u128)]
pub trait UInt: Int {
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
