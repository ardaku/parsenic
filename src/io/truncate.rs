use crate::io::Seek;

/// Trait providing a truncation operation.
pub trait Truncate: Seek {
    /// Truncate the destination at the current cursor position.
    fn truncate(&mut self);
}
