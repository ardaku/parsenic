use core::mem;

use crate::{seal::Seal, Reader};

/// Big endian reader extension trait
pub trait Read: Seal {
    /// Read the next big endian `u16`
    fn u16(&mut self) -> Option<u16>;

    /// Read the next big endian `u32`
    fn u32(&mut self) -> Option<u32>;

    /// Read the next big endian `u64`
    fn u64(&mut self) -> Option<u64>;

    /// Read the next big endian `u128`
    fn u128(&mut self) -> Option<u128>;
}

impl Read for Reader<'_> {
    fn u16(&mut self) -> Option<u16> {
        const SIZE: usize = mem::size_of::<u16>();

        let value = self.subslice(SIZE)?;

        Some(u16::from_be_bytes(value.get(..SIZE)?.try_into().ok()?))
    }

    fn u32(&mut self) -> Option<u32> {
        const SIZE: usize = mem::size_of::<u32>();

        let value = self.subslice(SIZE)?;

        Some(u32::from_be_bytes(value.get(..SIZE)?.try_into().ok()?))
    }

    fn u64(&mut self) -> Option<u64> {
        const SIZE: usize = mem::size_of::<u64>();

        let value = self.subslice(SIZE)?;

        Some(u64::from_be_bytes(value.get(..SIZE)?.try_into().ok()?))
    }

    fn u128(&mut self) -> Option<u128> {
        const SIZE: usize = mem::size_of::<u128>();

        let value = self.subslice(SIZE)?;

        Some(u128::from_be_bytes(value.get(..SIZE)?.try_into().ok()?))
    }
}
