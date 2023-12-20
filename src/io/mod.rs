//! I/O primitives

mod destination;
mod receiver;
mod seek;
mod sender;
mod source;
mod truncate;

pub use self::{
    destination::Destination, receiver::Receiver, seek::Seek, sender::Sender,
    source::Source, truncate::Truncate,
};
