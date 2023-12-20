use crate::{error::LenError, result::LenResult, Read};

/// [`slice`] [`Read`]er
#[derive(Debug)]
pub struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    /// Create a new `Reader` on the provided fixed-size `buffer`.
    pub fn new(buffer: &'a [u8]) -> Self {
        Self(buffer)
    }

    fn subslice<'b>(&mut self, len: usize) -> LenResult<&'b [u8]>
    where
        'a: 'b,
    {
        if len > self.0.len() {
            return Err(LenError);
        }

        let (slice, data) = self.0.split_at(len);

        self.0 = data;

        Ok(slice)
    }
}

impl Read for Reader<'_> {
    fn remaining(&self) -> usize {
        self.0.len()
    }

    fn bytes(&mut self, len: usize) -> LenResult<Self> {
        Ok(Reader(self.subslice(len)?))
    }

    fn slice(&mut self, len: usize) -> LenResult<&'_ [u8]> {
        self.subslice(len)
    }
}
