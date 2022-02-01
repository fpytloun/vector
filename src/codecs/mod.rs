//! A collection of codecs that can be used to transform between bytes streams /
//! byte messages, byte frames and structured events.

#![deny(missing_docs)]

pub mod decoding;
pub mod encoding;
mod ready_frames;

#[cfg(feature = "sources-syslog")]
pub use decoding::format::{SyslogDeserializer, SyslogDeserializerConfig};
pub use decoding::{
    format::{
        BytesDeserializer, BytesDeserializerConfig, JsonDeserializer, JsonDeserializerConfig,
    },
    framing::{
        BytesDecoder, BytesDecoderConfig, CharacterDelimitedDecoder,
        CharacterDelimitedDecoderConfig, LengthDelimitedDecoder, LengthDelimitedDecoderConfig,
        NewlineDelimitedDecoder, NewlineDelimitedDecoderConfig, OctetCountingDecoder,
        OctetCountingDecoderConfig,
    },
    Decoder,
};
pub use encoding::{
    format::{
        JsonSerializer, JsonSerializerConfig, RawMessageSerializer, RawMessageSerializerConfig,
    },
    framing::{
        CharacterDelimitedEncoder, CharacterDelimitedEncoderConfig, NewlineDelimitedEncoder,
        NewlineDelimitedEncoderConfig,
    },
};
pub use ready_frames::ReadyFrames;
