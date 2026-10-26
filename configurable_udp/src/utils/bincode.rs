//! bincode module

use serde::Serialize;
use serde::de::DeserializeOwned;

/// # Purpose function
///
/// The goal here is to define a data structure for encoding packets with information
/// Few things to identify:
/// - Size of the packet (Transmission mode specific network card of ethernet) (For IPv4, the
///   maximum payload size is 65535 octets minus the UDP header and minus the IP header because IPv4
///   has a 16-bit length field that also includes the header length.) source: <https://www.rfc-editor.org/info/rfc5426>/
/// - Structure of packets apart from UDP headers (Base information which can be overwritten with user defined constructs).

pub fn encode<T: Serialize>(data: &T) -> Vec<u8> {
    serde_json::to_vec(data).unwrap()
}

pub fn decode<T: DeserializeOwned>(bytes: &[u8]) -> T {
    serde_json::from_slice(bytes).unwrap()
}
