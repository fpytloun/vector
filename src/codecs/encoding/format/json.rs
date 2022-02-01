use bytes::BufMut;
use serde::{Deserialize, Serialize};

use super::{BoxedSerializer, Serializer, SerializerConfig};
use crate::event::Event;

/// Config used to build a `JsonSerializer`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JsonSerializerConfig;

impl JsonSerializerConfig {
    /// Creates a new `JsonSerializerConfig`.
    pub const fn new() -> Self {
        Self
    }
}

#[typetag::serde(name = "json")]
impl SerializerConfig for JsonSerializerConfig {
    fn build(&self) -> crate::Result<BoxedSerializer> {
        Ok(Box::new(JsonSerializer))
    }
}

/// Serializer that converts an `Event` to bytes using the JSON format.
#[derive(Debug, Clone)]
pub struct JsonSerializer;

impl JsonSerializer {
    /// Creates a new `JsonSerializer`.
    pub const fn new() -> Self {
        Self
    }
}

impl Serializer for JsonSerializer {
    fn serialize(&self, event: Event, buffer: &mut bytes::BytesMut) -> crate::Result<()> {
        let writer = buffer.writer();
        match event {
            Event::Log(log) => serde_json::to_writer(writer, &log),
            Event::Metric(metric) => serde_json::to_writer(writer, &metric),
        }
        .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Value;
    use bytes::BytesMut;
    use shared::btreemap;

    #[test]
    fn serialize_json() {
        let event = Event::from(btreemap! {
            "foo" => Value::from("bar")
        });
        let serializer = JsonSerializer::new();
        let mut bytes = BytesMut::new();

        serializer.serialize(event, &mut bytes).unwrap();

        assert_eq!(bytes.freeze(), r#"{"foo":"bar"}"#);
    }
}
