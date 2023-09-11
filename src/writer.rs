use alloc::vec::Vec;

use crate::{UInt, Write};

/// Writes to a buffer.
#[derive(Debug)]
pub struct Writer<'a>(&'a mut Vec<u8>);

impl<'a> Writer<'a> {
    /// Create a new `Writer` to the provided `buffer`.
    pub fn new(buffer: &'a mut Vec<u8>) -> Self {
        Self(buffer)
    }
}

impl Write for Writer<'_> {
    fn uleb128<T: UInt>(&mut self, value: T) {
        let mut remaining = value;

        while {
            let byte = remaining.little();

            remaining >>= 7;

            let more = remaining != T::ZERO;

            self.u8(if more { byte | 0x80 } else { byte & !0x80 });

            more
        } {}
    }

    fn u8(&mut self, byte: u8) {
        self.0.push(byte);
    }

    fn i8(&mut self, byte: i8) {
        let [byte] = byte.to_ne_bytes();

        self.0.push(byte);
    }

    fn str(&mut self, string: impl AsRef<str>) {
        self.bytes(string.as_ref().as_bytes())
    }

    fn bytes(&mut self, bytes: impl AsRef<[u8]>) {
        self.0.extend(bytes.as_ref())
    }
}
