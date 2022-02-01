use bytes::{BufMut, BytesMut};
use serde::{Deserialize, Serialize};

use super::{BoxedFramer, BoxedFramingError, Framer, FramingConfig};

/// Config used to build a `CharacterDelimitedEncoder`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CharacterDelimitedEncoderConfig {
    character_delimited: CharacterDelimitedEncoderOptions,
}

/// Options for building a `CharacterDelimitedEncoder`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CharacterDelimitedEncoderOptions {
    /// The character that delimits byte sequences.
    delimiter: u8,
}

#[typetag::serde(name = "character_delimited")]
impl FramingConfig for CharacterDelimitedEncoderConfig {
    fn build(&self) -> crate::Result<BoxedFramer> {
        Ok(Box::new(CharacterDelimitedEncoder::new(
            self.character_delimited.delimiter,
        )))
    }
}

/// An encoder for handling bytes that are delimited by (a) chosen character(s).
#[derive(Debug, Clone)]
pub struct CharacterDelimitedEncoder {
    delimiter: u8,
}

impl CharacterDelimitedEncoder {
    /// Creates a `CharacterDelimitedEncoder` with the specified delimiter.
    pub const fn new(delimiter: u8) -> Self {
        Self { delimiter }
    }
}

impl Framer for CharacterDelimitedEncoder {
    fn frame(&self, buffer: &mut BytesMut) -> Result<(), BoxedFramingError> {
        buffer.put_u8(self.delimiter);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let codec = CharacterDelimitedEncoder::new(b'\n');

        let mut buf = BytesMut::from("abc");
        codec.frame(&mut buf).unwrap();

        assert_eq!(b"abc\n", &buf[..]);
    }
}
