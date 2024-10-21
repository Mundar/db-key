use bincode::Options;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::io::Write;

fn key_serializer() -> impl bincode::Options {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .with_big_endian()
        .reject_trailing_bytes()
}

pub trait SerializableKey: serde::Serialize + serde::de::DeserializeOwned {
    type RawKey: Default + Write + AsRef<[u8]>;

    fn serialize(&self) -> Result<Self::RawKey, bincode::Error> {
        let mut key = Default::default();
        key_serializer().serialize_into(&mut key, self)?;
        Ok(key)
    }

    fn deserialize<'a>(raw_key: &'a [u8]) -> Result<Self, bincode::Error> {
        Ok(key_serializer().deserialize_from(raw_key)?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeKey {
    pub id: u64,
    pub index: u32,
}

pub type SerdeRawKey = SmallVec<[u8; SerdeKey::SERIALIZED_SIZE]>;
impl SerdeKey {
    const SERIALIZED_SIZE: usize = 16;

    pub const fn new(
        id: u64,
        index: u32,
    ) -> SerdeKey {
        SerdeKey {
            id,
            index,
        }
    }
}

impl SerializableKey for SerdeKey {
    type RawKey = SerdeRawKey;
}
