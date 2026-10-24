use serde::de::DeserializeOwned;
use serde::Serialize;

/// The goal here is to define a data structure for encoding packets with information
/// Few things to identify:
/// - Size of the packet (Transmission mode specific network card of ethernet) (For IPv4, the
///   maximum payload size is 65535 octets minus the UDP header and minus the IP header because IPv4
///   has a 16-bit length field that also includes the header length.) source: <https://www.rfc-editor.org/info/rfc5426>/
/// - Structure of packets apart from UDP headers (Base information which can be overwritten with user defined constructs).

/// Definition for reliable packet transmission
/// - Assumption 2 nodes know each other.
/// - The packet transmission starts from 0. (Pre-computed on the transmitter for number of packets needed).
/// - The data size is user defined.
/// https://git.sr.ht/~stygianentity/bincode (Used for encoding and decoding bytes)
/// TODO: To write custom serialise library in rust
/// takes any type and returns it as encoded bytes
/// todo: look into upgrading to (<https://docs.rs/bincode/2.0.0-rc.3/bincode/index.html>)
/// Get rid of serde "Starting from bincode 2, serde is now an optional dependency.
/// If you want to use serde, please enable the serde feature. See Features for more
/// information."
pub fn encode<T: Serialize>(value: T) -> Vec<u8> {
    let encoded: Vec<u8> = bincode::serialize(&value).unwrap();
    encoded
}

pub fn decode<T: DeserializeOwned>(value: &[u8]) -> T {
    let data = bincode::deserialize(&value).unwrap();
    data
}
