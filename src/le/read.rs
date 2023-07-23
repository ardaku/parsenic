use core::mem;

use traitful::extend;

use crate::Reader;

/// Little endian reader extension trait
#[extend(Reader<'_>)]
pub trait Read {
    /// Read the next little endian `u16`
    fn u16(&mut self) -> Option<u16> {
        const SIZE: usize = mem::size_of::<u16>();

        let value = self.subslice(SIZE)?;

        Some(u16::from_le_bytes(value.get(..SIZE)?.try_into().ok()?))
    }

    /// Read the next little endian `u32`
    fn u32(&mut self) -> Option<u32> {
        const SIZE: usize = mem::size_of::<u32>();

        let value = self.subslice(SIZE)?;

        Some(u32::from_le_bytes(value.get(..SIZE)?.try_into().ok()?))
    }

    /// Read the next little endian `u64`
    fn u64(&mut self) -> Option<u64> {
        const SIZE: usize = mem::size_of::<u64>();

        let value = self.subslice(SIZE)?;

        Some(u64::from_le_bytes(value.get(..SIZE)?.try_into().ok()?))
    }

    /// Read the next little endian `u128`
    fn u128(&mut self) -> Option<u128> {
        const SIZE: usize = mem::size_of::<u128>();

        let value = self.subslice(SIZE)?;

        Some(u128::from_le_bytes(value.get(..SIZE)?.try_into().ok()?))
    }
}
