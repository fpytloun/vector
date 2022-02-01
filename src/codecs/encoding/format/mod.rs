//! A collection of formats that can be used to convert between structured
//! events and byte frames.

#![deny(missing_docs)]

mod json;
mod raw_message;

pub use json::{JsonSerializer, JsonSerializerConfig};
pub use raw_message::{RawMessageSerializer, RawMessageSerializerConfig};

use crate::event::Event;
use bytes::BytesMut;
use dyn_clone::DynClone;
use std::fmt::Debug;

/// Serialize a structured event into a byte frame.
pub trait Serializer: DynClone + Debug + Send + Sync {
    /// Serialize an event into the provided buffer.
    fn serialize(&self, event: Event, buffer: &mut BytesMut) -> crate::Result<()>;
}

impl tokio_util::codec::Encoder<Event> for dyn Serializer {
    type Error = crate::Error;

    fn encode(&mut self, item: Event, dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.serialize(item, dst)
    }
}

dyn_clone::clone_trait_object!(Serializer);

/// A `Box` containing a `Serializer`.
pub type BoxedSerializer = Box<dyn Serializer>;

/// Define options for a serializer and build it from the config object.
///
/// Implementors must annotate the struct with `#[typetag::serde(name = "...")]`
/// to define which value should be read from the `codec` key to select their
/// implementation.
#[typetag::serde(tag = "codec")]
pub trait SerializerConfig: Debug + DynClone + Send + Sync {
    /// Builds a serializer from this configuration.
    ///
    /// Fails if the configuration is invalid.
    fn build(&self) -> crate::Result<BoxedSerializer>;
}

dyn_clone::clone_trait_object!(SerializerConfig);
