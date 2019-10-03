use std::str::Utf8Error;
use std::string::FromUtf8Error;

use crate::proto::kv;

#[derive(Clone, Debug)]
pub struct KeyValue {
    key: Vec<u8>,
    value: Vec<u8>,
    version: i64,
    create_revision: i64,
    mod_revision: i64,
    lease: i64,
}

impl KeyValue {
    pub fn key(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.key)
    }

    pub fn raw_key(&self) -> &[u8] {
        &self.key
    }

    pub fn into_key(self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.key)
    }

    pub fn into_raw_key(self) -> Vec<u8> {
        self.key
    }

    pub fn value(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.value)
    }

    pub fn raw_value(&self) -> &[u8] {
        &self.value
    }

    pub fn into_value(self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.value)
    }

    pub fn into_raw_value(self) -> Vec<u8> {
        self.value
    }

    pub fn into_key_value(self) -> Result<(String, String), FromUtf8Error> {
        let key = String::from_utf8(self.key)?;
        let value = String::from_utf8(self.value)?;

        Ok((key, value))
    }

    pub fn into_raw_key_value(self) -> (Vec<u8>, Vec<u8>) {
        (self.key, self.value)
    }

    pub fn version(&self) -> i64 {
        self.version
    }

    pub fn create_revision(&self) -> i64 {
        self.create_revision
    }

    pub fn mod_revision(&self) -> i64 {
        self.mod_revision
    }

    pub fn lease(&self) -> i64 {
        self.lease
    }
}

impl From<kv::KeyValue> for KeyValue {
    fn from(kv: kv::KeyValue) -> Self {
        KeyValue {
            key: kv.key,
            value: kv.value,
            version: kv.version,
            create_revision: kv.create_revision,
            mod_revision: kv.mod_revision,
            lease: kv.lease,
        }
    }
}
