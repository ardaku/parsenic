use traitful::extend;

use crate::Reader;

/// Big endian reader extension trait
#[extend(Reader<'_>)]
pub trait Read {
    /// Read the next big endian `u16`
    fn u16(&mut self) -> Option<u16> {
        Some(u16::from_be_bytes(self.take()?))
    }

    /// Read the next big endian `u32`
    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_be_bytes(self.take()?))
    }

    /// Read the next big endian `u64`
    fn u64(&mut self) -> Option<u64> {
        Some(u64::from_be_bytes(self.take()?))
    }

    /// Read the next big endian `u128`
    fn u128(&mut self) -> Option<u128> {
        Some(u128::from_be_bytes(self.take()?))
    }
}
