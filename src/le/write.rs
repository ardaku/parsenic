use traitful::extend;

use crate::Writer;

/// Little endian writer extension trait
#[extend(Writer<'_>)]
pub trait Write {
    /// Write out a little endian encoded 2-byte integer.
    fn u16(&mut self, int: u16) {
        self.bytes(int.to_le_bytes());
    }

    /// Write out a little endian encoded 4-byte integer.
    fn u32(&mut self, int: u32) {
        self.bytes(int.to_le_bytes());
    }

    /// Write out a little endian encoded 8-byte integer.
    fn u64(&mut self, int: u64) {
        self.bytes(int.to_le_bytes());
    }

    /// Write out a little endian encoded 16-byte integer.
    fn u128(&mut self, int: u128) {
        self.bytes(int.to_le_bytes());
    }
}
