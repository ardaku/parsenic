//! #### A simple no-std I/O and parsing crate

#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg",
    html_root_url = "https://docs.rs/parsenic"
)]
#![no_std]
#![forbid(unsafe_code, missing_docs)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

pub mod be;
pub mod buf;
pub mod class;
mod empty;
pub mod error;
pub mod io;
pub mod le;
mod purge;
mod read;
mod reader;
pub mod result;
mod write;
mod writer;

pub use self::{
    empty::{empty, Empty},
    error::Error,
    purge::{purge, Purge},
    read::Read,
    reader::Reader,
    result::Result,
    write::Write,
    writer::Writer,
};
