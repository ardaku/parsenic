//! `Result` type aliases

use crate::error;

/// Type alias for `Result` of [`error::Error`]
pub type Result<T = (), E = error::Error> = core::result::Result<T, E>;

/// Type alias for `Result` of [`error::Len`]
pub type Len<T = (), E = error::Len> = Result<T, E>;

/// Type alias for `Result` of [`error::End`]
pub type End<T = (), E = error::End> = Result<T, E>;

/// Type alias for `Result` of [`error::Utf8`]
pub type Utf8<T = (), E = error::Utf8> = Result<T, E>;

/// Type alias for `Result` of [`error::Str`]
pub type Str<T = (), E = error::Str> = Result<T, E>;
