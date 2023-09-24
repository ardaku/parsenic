use traitful::extend;

use crate::result::FlushResult;

/// Little endian writer extension trait
#[extend]
pub trait Write: crate::Write {
    /// Write out a little endian encoded 2-byte unsigned integer.
    fn u16(&mut self, int: u16) -> FlushResult {
        self.bytes(int.to_le_bytes())
    }

    /// Write out a little endian encoded 4-byte unsigned integer.
    fn u32(&mut self, int: u32) -> FlushResult {
        self.bytes(int.to_le_bytes())
    }

    /// Write out a little endian encoded 8-byte unsigned integer.
    fn u64(&mut self, int: u64) -> FlushResult {
        self.bytes(int.to_le_bytes())
    }

    /// Write out a little endian encoded 16-byte unsigned integer.
    fn u128(&mut self, int: u128) -> FlushResult {
        self.bytes(int.to_le_bytes())
    }

    /// Write out a little endian encoded 2-byte signed integer.
    fn i16(&mut self, int: i16) -> FlushResult {
        self.bytes(int.to_le_bytes())
    }

    /// Write out a little endian encoded 4-byte signed integer.
    fn i32(&mut self, int: i32) -> FlushResult {
        self.bytes(int.to_le_bytes())
    }

    /// Write out a little endian encoded 8-byte signed integer.
    fn i64(&mut self, int: i64) -> FlushResult {
        self.bytes(int.to_le_bytes())
    }

    /// Write out a little endian encoded 16-byte signed integer.
    fn i128(&mut self, int: i128) -> FlushResult {
        self.bytes(int.to_le_bytes())
    }
}
