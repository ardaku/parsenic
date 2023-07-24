//! `Result` type aliases

use crate::error::{EndError, Error, LenError, StrError, Utf8Error};

/// Type alias for `Result` of [`Error`]
pub type Result<T = (), E = Error> = core::result::Result<T, E>;

/// Type alias for `Result` of [`LenError`]
pub type LenResult<T = (), E = LenError> = Result<T, E>;

/// Type alias for `Result` of [`EndError`]
pub type EndResult<T = (), E = EndError> = Result<T, E>;

/// Type alias for `Result` of [`Utf8Error`]
pub type Utf8Result<T = (), E = Utf8Error> = Result<T, E>;

/// Type alias for `Result` of [`StrError`]
pub type StrResult<T = (), E = StrError> = Result<T, E>;
