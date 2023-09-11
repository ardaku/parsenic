use core::iter::Extend;

use crate::{UInt, Write};

/// Writes to a buffer.
#[derive(Debug)]
pub struct Writer<'a, T: Extend<u8>>(&'a mut T);

impl<'a, T: Extend<u8>> Writer<'a, T> {
    /// Create a new `Writer` to the provided `buffer`.
    pub fn new(buffer: &'a mut T) -> Self {
        Self(buffer)
    }
}

impl<T: Extend<u8>> Write for Writer<'_, T> {
    fn uleb128<V: UInt>(&mut self, value: V) {
        let mut remaining = value;

        while {
            let byte = remaining.little();

            remaining >>= 7;

            let more = remaining != V::ZERO;

            self.u8(if more { byte | 0x80 } else { byte & !0x80 });

            more
        } {}
    }

    fn u8(&mut self, byte: u8) {
        self.0.extend(Some(byte));
    }

    fn i8(&mut self, byte: i8) {
        let [byte] = byte.to_ne_bytes();

        self.0.extend(Some(byte));
    }

    fn str(&mut self, string: impl AsRef<str>) {
        self.bytes(string.as_ref().as_bytes())
    }

    fn bytes(&mut self, bytes: impl AsRef<[u8]>) {
        self.0.extend(bytes.as_ref().into_iter().cloned())
    }
}
