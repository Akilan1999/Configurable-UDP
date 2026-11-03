//! Encode Decode module.

use crate::error;
use bincode::{
    Encode,
    config::{Configuration, Fixint, LittleEndian, NoLimit},
};
use core::cell::LazyCell;
use crate::error::Error;

/// Default Configuration for encoding and decoding.
const CONFIG: LazyCell<Configuration<LittleEndian, Fixint, NoLimit>> = LazyCell::new(|| {
    bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding()
});

#[inline]
pub(crate) fn encode<'a, T>(value: T, slice: &'a mut [u8]) -> Result<EncodedData<'a>, error::Error>
where
    T: Encode,
{
    bincode::encode_into_slice(&value, slice, *CONFIG)
        .map_err(error::Error::Encode)
        .map(|len| EncodedData { slice, len })
}

#[derive(Debug)]
pub struct EncodedData<'a> {
    slice: &'a [u8],
    len: usize,
}

impl<'a> EncodedData<'a> {
    pub fn get_slice(&self) -> Option<&[u8]> {
        self.slice.get(..self.len)
    }
}

pub fn encode_vec<T: bincode::Encode>(data: T) ->  Result<Vec<u8>, Error> {
    bincode::encode_to_vec(&data, *CONFIG).map_err(Error::Encode)
}


/// # Purpose of function
///
/// decoding function.
///
///
/// # Errors
///
/// It errors when the the value is corrupted or there is a configuration mismatch.
///
#[inline]
pub fn decode<T, V>(value: V) -> Result<T, error::Error>
where
    V: AsRef<[u8]>,
    T: bincode::Decode<()>,
{
    bincode::decode_from_slice(value.as_ref(), *CONFIG)
        .map_err(error::Error::Decode)
        .map(|(v, _)| v)
}

/*

pub fn encode<T: Serialize>(data: &T) -> Vec<u8> {
    serde_json::to_vec(data).unwrap()
}

pub fn decode<T: DeserializeOwned>(bytes: &[u8]) -> T {
    serde_json::from_slice(bytes).unwrap()
// >>>>>>> origin/master
}
*/
