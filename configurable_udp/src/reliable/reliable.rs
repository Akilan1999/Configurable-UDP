//! Reliability module.

use bincode::{Decode, Encode};


#[expect(dead_code, reason="still prototyping")]
/// Constant.
const N: u128 = 1000;

/// defines a result queue data packet.

#[derive(Encode)]
pub struct DataPacketStructure {
    /// identity value.
    id: u8,
    /// sequence number.
    seq_no: u8,
    /// Data being transferred.
    data: Vec<u8>,
}

/// Defines number of packets.
#[derive(Encode, Decode)]
pub struct NumberOfPackets {
    /// concrete length.
    packet_length: u8,
}

/// Type of packet being transmitted.
#[non_exhaustive]
pub enum PacketType {
    /// Defines the inner packet structure.
    DataPacketStructure(DataPacketStructure),
    /// Defines the number of packets.
    NumberOfPackets(NumberOfPackets),
}
/// Creates a Data packet from a vector of bytes.
pub fn create_data_packet(data: Vec<u8>) -> DataPacketStructure {
    DataPacketStructure{
        id: 0,
        seq_no: 0,
        data,
    }
}

/// Matching on packet type.
pub fn type_match_packet(bytes: &[u8]) {
    println!("bytes {bytes:?}");
    // decode udp packet attributes
    // let d: DataPacketStructure = decode(bytes);

}

