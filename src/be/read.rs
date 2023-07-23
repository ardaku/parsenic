use traitful::extend;

use crate::{result, Reader};

/// Big endian reader extension trait
#[extend(Reader<'_>)]
pub trait Read {
    /// Read the next big endian `u16`
    fn u16(&mut self) -> result::Len<u16> {
        Ok(u16::from_be_bytes(self.take()?))
    }

    /// Read the next big endian `u32`
    fn u32(&mut self) -> result::Len<u32> {
        Ok(u32::from_be_bytes(self.take()?))
    }

    /// Read the next big endian `u64`
    fn u64(&mut self) -> result::Len<u64> {
        Ok(u64::from_be_bytes(self.take()?))
    }

    /// Read the next big endian `u128`
    fn u128(&mut self) -> result::Len<u128> {
        Ok(u128::from_be_bytes(self.take()?))
    }
}
