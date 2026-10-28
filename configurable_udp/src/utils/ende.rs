//! Encode Decode module.

use crate::error;
use bincode::{
    Encode,
    config::{Configuration, Fixint, LittleEndian, NoLimit},
};
use core::cell::LazyCell;

/// Default Configuration for encoding and decoding.
const CONFIG: LazyCell<Configuration<LittleEndian, Fixint, NoLimit>> = LazyCell::new(|| {
    bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding()
});

/// # Purpose function
///
/// The goal here is to define a data structure for encoding packets with information
/// Few things to identify:
/// - Size of the packet (Transmission mode specific network card of ethernet) (For IPv4, the
///   maximum payload size is 65535 octets minus the UDP header and minus the IP header because IPv4
///   has a 16-bit length field that also includes the header length.) source: <https://www.rfc-editor.org/info/rfc5426>/
/// - Structure of packets apart from UDP headers (Base information which can be overwritten with user defined constructs).
///
/// Definition for reliable packet transmission
/// - Assumption 2 nodes know each other.
/// - The packet transmission starts from 0. (Pre-computed on the transmitter for number of packets needed).
/// - The data size is user defined.
///   <https://git.sr.ht/~stygianentity/bincode> (Used for encoding and decoding bytes)
///   TODO: To write custom serialise library in rust
///   takes any type and returns it as encoded bytes
///   todo: look into upgrading to (<https://docs.rs/bincode/2.0.0-rc.3/bincode/index.html>)
#[inline]
pub(crate) fn encode<T>(value: T, slice: &mut [u8]) -> Result<usize, error::Error>
where
    T: Encode,
{
    bincode::encode_into_slice(&value, slice, *CONFIG).map_err(error::Error::Encode)
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
pub fn decode<T>(value: &[u8]) -> Result<(T, usize), error::Error>
where
    T: bincode::Decode<()>,
{
    bincode::decode_from_slice(value, *CONFIG).map_err(error::Error::Decode)
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
