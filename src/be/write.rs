use crate::{seal::Seal, Writer};

/// Big endian writer extension trait
pub trait Write: Seal {
    /// Write out a big endian encoded 2-byte integer.
    fn u16(&mut self, int: u16);

    /// Write out a big endian encoded 4-byte integer.
    fn u32(&mut self, int: u32);

    /// Write out a big endian encoded 8-byte integer.
    fn u64(&mut self, int: u64);

    /// Write out a big endian encoded 16-byte integer.
    fn u128(&mut self, int: u128);
}

impl Write for Writer<'_> {
    fn u16(&mut self, int: u16) {
        self.bytes(int.to_be_bytes());
    }

    fn u32(&mut self, int: u32) {
        self.bytes(int.to_be_bytes());
    }

    fn u64(&mut self, int: u64) {
        self.bytes(int.to_be_bytes());
    }

    fn u128(&mut self, int: u128) {
        self.bytes(int.to_be_bytes());
    }
}
