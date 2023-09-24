use super::Destination;

/// A buffered writer
pub struct BufWriter<T>(T);

impl<T> BufWriter<T>
where
    T: Destination,
{
    /// Create a new `BufWriter<T>`
    pub fn new(dst: T) -> Self {
        Self(dst)
    }
}
