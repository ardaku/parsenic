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
// mod buf_writer;
pub mod buf;
pub mod class;
mod cursor;
mod empty;
pub mod error;
pub mod le;
mod purge;
mod read;
mod reader;
pub mod result;
mod seek;
mod write;
mod writer;

pub use self::{
    // buf_writer::BufWriter,
    cursor::Cursor,
    empty::{empty, Empty},
    error::Error,
    purge::{purge, Purge},
    read::Read,
    reader::Reader,
    result::Result,
    seek::Seek,
    write::Write,
    writer::Writer,
};
