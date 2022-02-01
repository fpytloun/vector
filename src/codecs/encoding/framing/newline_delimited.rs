use bytes::BytesMut;
use serde::{Deserialize, Serialize};

use super::{BoxedFramer, BoxedFramingError, CharacterDelimitedEncoder, Framer, FramingConfig};

/// Config used to build a `NewlineDelimitedEncoder`.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct NewlineDelimitedEncoderConfig;

impl NewlineDelimitedEncoderConfig {
    /// Creates a new `NewlineDelimitedEncoderConfig`.
    pub fn new() -> Self {
        Default::default()
    }
}

#[typetag::serde(name = "newline_delimited")]
impl FramingConfig for NewlineDelimitedEncoderConfig {
    fn build(&self) -> crate::Result<BoxedFramer> {
        Ok(Box::new(NewlineDelimitedEncoder::new()))
    }
}

/// A codec for handling bytes that are delimited by (a) newline(s).
#[derive(Debug, Clone)]
pub struct NewlineDelimitedEncoder(CharacterDelimitedEncoder);

impl NewlineDelimitedEncoder {
    /// Creates a new `NewlineDelimitedEncoder`.
    pub const fn new() -> Self {
        Self(CharacterDelimitedEncoder::new(b'\n'))
    }
}

impl Framer for NewlineDelimitedEncoder {
    fn frame(&self, buffer: &mut BytesMut) -> Result<(), BoxedFramingError> {
        self.0.frame(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_bytes() {
        let mut input = BytesMut::from("foo");
        let encoder = NewlineDelimitedEncoder::new();

        encoder.frame(&mut input).unwrap();

        assert_eq!(input, "foo\n");
    }
}
