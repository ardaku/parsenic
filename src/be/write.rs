use traitful::extend;

use crate::Writer;

/// Big endian writer extension trait
#[extend(Writer<'_>)]
pub trait Write {
    /// Write out a big endian encoded 2-byte unsigned integer.
    fn u16(&mut self, int: u16) {
        self.bytes(int.to_be_bytes());
    }

    /// Write out a big endian encoded 4-byte unsigned integer.
    fn u32(&mut self, int: u32) {
        self.bytes(int.to_be_bytes());
    }

    /// Write out a big endian encoded 8-byte unsigned integer.
    fn u64(&mut self, int: u64) {
        self.bytes(int.to_be_bytes());
    }

    /// Write out a big endian encoded 16-byte unsigned integer.
    fn u128(&mut self, int: u128) {
        self.bytes(int.to_be_bytes());
    }

    /// Write out a big endian encoded 2-byte signed integer.
    fn i16(&mut self, int: i16) {
        self.bytes(int.to_be_bytes());
    }

    /// Write out a big endian encoded 4-byte signed integer.
    fn i32(&mut self, int: i32) {
        self.bytes(int.to_be_bytes());
    }

    /// Write out a big endian encoded 8-byte signed integer.
    fn i64(&mut self, int: i64) {
        self.bytes(int.to_be_bytes());
    }

    /// Write out a big endian encoded 16-byte signed integer.
    fn i128(&mut self, int: i128) {
        self.bytes(int.to_be_bytes());
    }
}
