//! A collection of framing methods that can be used to convert between byte
//! chunks and byte frames with defined boundaries.

#![deny(missing_docs)]

mod character_delimited;
mod newline_delimited;

pub use character_delimited::{CharacterDelimitedEncoder, CharacterDelimitedEncoderConfig};
pub use newline_delimited::{NewlineDelimitedEncoder, NewlineDelimitedEncoderConfig};

use bytes::BytesMut;
use dyn_clone::DynClone;
use std::fmt::Debug;
use tokio_util::codec::LinesCodecError;

/// An error that occurred while framing bytes.
pub trait FramingError: std::error::Error + Send + Sync {}

impl std::error::Error for BoxedFramingError {}

impl FramingError for std::io::Error {}

impl FramingError for LinesCodecError {}

impl From<std::io::Error> for BoxedFramingError {
    fn from(error: std::io::Error) -> Self {
        Box::new(error)
    }
}

/// A `Box` containing a `FramingError`.
pub type BoxedFramingError = Box<dyn FramingError>;

/// Wrap bytes into a frame.
pub trait Framer: DynClone + Debug + Send + Sync {
    /// Wrap the buffer into a byte frame.
    fn frame(&self, buffer: &mut BytesMut) -> Result<(), BoxedFramingError>;
}

impl tokio_util::codec::Encoder<()> for dyn Framer {
    type Error = BoxedFramingError;

    fn encode(&mut self, _: (), dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.frame(dst)
    }
}

dyn_clone::clone_trait_object!(Framer);

/// A `Box` containing a `Framer`.
pub type BoxedFramer = Box<dyn Framer>;

/// Define options for a framer and build it from the config object.
///
/// Implementors must annotate the struct with `#[typetag::serde(name = "...")]`
/// to define which value should be read from the `method` key to select their
/// implementation.
#[typetag::serde(tag = "method")]
pub trait FramingConfig: Debug + DynClone + Send + Sync {
    /// Builds a framer from this configuration.
    ///
    /// Fails if the configuration is invalid.
    fn build(&self) -> crate::Result<BoxedFramer>;
}

dyn_clone::clone_trait_object!(FramingConfig);
