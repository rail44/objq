use std::io::Read;
use serde::de;
use rmp_serde;
use rmp_serde::decode::Deserializer;
use super::Value;

#[derive(Debug)]
pub struct MessagePack;

impl MessagePack {
    pub fn new() -> MessagePack {
        MessagePack
    }
}

pub type Error = rmp_serde::decode::Error;

impl super::Input for MessagePack {
    type Error = Error;

    fn input<T: Read>(self, r: &mut T) -> Result<Value, Self::Error> {
        let mut de = Deserializer::new(r);
        de::Deserialize::deserialize(&mut de)
    }
}
