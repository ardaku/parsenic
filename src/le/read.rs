use traitful::extend;

use crate::{result::LenResult, Reader};

/// Little endian reader extension trait
#[extend(Reader<'_>)]
pub trait Read {
    /// Read the next little endian `u16`
    fn u16(&mut self) -> LenResult<u16> {
        Ok(u16::from_le_bytes(self.take()?))
    }

    /// Read the next little endian `u32`
    fn u32(&mut self) -> LenResult<u32> {
        Ok(u32::from_le_bytes(self.take()?))
    }

    /// Read the next little endian `u64`
    fn u64(&mut self) -> LenResult<u64> {
        Ok(u64::from_le_bytes(self.take()?))
    }

    /// Read the next little endian `u128`
    fn u128(&mut self) -> LenResult<u128> {
        Ok(u128::from_le_bytes(self.take()?))
    }
}
