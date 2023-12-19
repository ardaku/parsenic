/// Reader that is always at EOF
///
/// Returned by [`empty()`].
#[non_exhaustive]
#[derive(Copy, Clone, Default, Debug)]
pub struct Empty;

/// Construct a new handle to an empty reader.
///
/// The returned type implements [`Read`](crate::Read) as an extension trait
/// on an [`Iterator`] of [`u8`]s.
///
/// This is essentially equivalent to [`core::iter::empty()`], but without the
/// generic.  
///
/// This API takes some inspiration from [`std::io::empty()`].
///
/// [`std::io::empty()`]: https://doc.rust-lang.org/stable/std/io/fn.empty.html
pub fn empty() -> Empty {
    Empty
}

impl Iterator for Empty {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

impl ExactSizeIterator for Empty {
    fn len(&self) -> usize {
        0
    }
}
