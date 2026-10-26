use serde::{Deserialize, Serialize};
use crate::utils::bincode::decode;

const n: u128 = 1000;


// ------------------ Packet IDs -------------------
const DATA_PACKET_STRUCTURE_CODE: u64 = 1;
const NUMBER_OF_PACKETS_CODE: u64 = 2;
// -------------------------------------------------

// define a result queue

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPacketStructure {
    packet_type: u64,
    seq_no: u64,
    data: Vec<u8>,
}

#[derive(Serialize,Deserialize, Debug)]
pub struct NumberOfPackets {
    packet_type: u64,
    packet_length: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PacketType {
    DataPacketStructure(DataPacketStructure),
    NumberOfPackets(NumberOfPackets),
}

pub fn create_data_packet(data: Vec<u8>) -> DataPacketStructure {
    DataPacketStructure {
        packet_type: DATA_PACKET_STRUCTURE_CODE,
        seq_no: 1,
        data,
    }
}

pub fn type_match_packet(bytes: &[u8]) {
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

