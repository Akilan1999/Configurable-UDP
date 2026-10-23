// The goal here is to define a data structure for encoding packets with information
// Few things to identify:
// - Size of the packet (Transmission mode specific network card of ethernet) (For IPv4, the
//   maximum payload size is 65535 octets minus the UDP header and minus the IP header because IPv4
//   has a 16-bit length field that also includes the header length.) source: https://www.rfc-editor.org/info/rfc5426/
// - Structure of packets apart from UDP headers (Base information which can be overwritten with user defined constructs).

use std::arch::aarch64::veor3q_s8;
use std::io::Bytes;
use serde::Serialize;

// Definition for reliable packet transmission
// - Assumption 2 nodes know each other.
// - The packet transmission starts from 0. (Pre-computed on the transmitter for number of packets needed).
// - The data size is user defined.

// https://git.sr.ht/~stygianentity/bincode (Used for encoding and decoding bytes)
// TODO: To write custom serialise library in rust
const n: u128 = 1000;

#[derive(Serialize)]
pub struct DataPacketStructure {
    id: u8,
    seq_no: u8,
    data: Vec<u8>,
}

#[derive(Serialize)]
pub struct NumberOfPackets {
    packet_length: u8,
}

pub enum PacketType {
    // DataPacketStructure(DataPacketStructure),
    // NumberOfPackets(NumberOfPackets),
}

pub fn create_data_packet(data: Vec<u8>) -> DataPacketStructure {
    DataPacketStructure{
        id: 0,
        seq_no: 0,
        data,
    }
}

// takes any type and returns it as encoded bytes
// todo: look into upgrading to (https://docs.rs/bincode/2.0.0-rc.3/bincode/index.html)
// Get rid of serde "Starting from bincode 2, serde is now an optional dependency.
// If you want to use serde, please enable the serde feature. See Features for more
// information."
pub fn encode_packet<T: Serialize>(value: T) -> Vec<u8> {
    let encoded: Vec<u8> = bincode::serialize(&value).unwrap();
    encoded
}

pub fn type_match_packet(bytes: &[u8]) {
    // decode udp packet attributes

}