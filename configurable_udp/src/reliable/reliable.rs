// <<<<<<< HEAD
//! Reliability module.

use serde::{Deserialize, Serialize};
use bincode::{Decode, Encode};


#[expect(dead_code, reason="still prototyping")]
/// Constant.
const N: u128 = 1000;


/// Defines number of packets.
#[derive(Encode, Decode)]
pub struct NumberOfPackets {
    /// concrete length.
    packet_length: u8,
    /// type of packet.
    packet_type: u64,
}



// ------------------ Packet IDs -------------------
const DATA_PACKET_STRUCTURE_CODE: u64 = 1;
const NUMBER_OF_PACKETS_CODE: u64 = 2;
// -------------------------------------------------

// define a result queue

#[derive(Serialize, Deserialize, Debug, Encode, Decode)]
pub struct DataPacketStructure {
    /// type of packet.
    packet_type: u64,
    /// sequence number.
    seq_no: u64,
    /// inner data.
    data: Vec<u8>,
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
    DataPacketStructure {
        packet_type: DATA_PACKET_STRUCTURE_CODE,
        seq_no: 1,
        data,
    }
}

/// Matching on packet type.
pub fn type_match_packet_serde(bytes: &[u8]) {
    let value: serde_json::Value = serde_json::from_slice(bytes).unwrap();

    let id = value["packet_type"]
        .as_u64()
        .ok_or("Missing packet packet_type");

    println!("{id:?}");

    match id {
        Ok(DATA_PACKET_STRUCTURE_CODE) => {println!("Received DataPacketStructure");}
        Ok(NUMBER_OF_PACKETS_CODE) => {println!("Received NumberOfPackets");}
        _ => {}
    }

    // match id {
    //     12 => Ok(DataPacketStructure(
    //         serde_json::from_value(value)?
    //     )),
    //     13 => Ok(PacketType::NumberOfPackets(
    //         serde_json::from_value(value)?
    //     )),
    //     _ => Err(format!("Unknown packet ID: {id}").into()),
    // }
}

