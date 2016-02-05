use std::io::Write;
use rmp_serde;
use rmp_serde::encode::Serializer;
use serde::ser;
use serde_json::Value;

#[derive(Debug)]
pub struct MessagePack;

impl MessagePack {
    pub fn new(_: Vec<&str>) -> MessagePack {
        MessagePack
    }
}

pub type Error = rmp_serde::encode::Error;

impl super::Output for MessagePack {
    type Error = Error;

    fn output<T: Write>(self, w: &mut T, value: &Value) -> Result<(), Self::Error> {
        let mut ser = Serializer::new(w);
        ser::Serialize::serialize(value, &mut ser)
    }
}
