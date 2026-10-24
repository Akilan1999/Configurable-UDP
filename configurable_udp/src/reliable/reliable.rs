use serde::{Deserialize, Serialize};

#[allow(dead_code)]
const N: u128 = 1000;

// define a result queue 

#[derive(Serialize)]
pub struct DataPacketStructure {
    id: u8,
    seq_no: u8,
    data: Vec<u8>,
}

#[derive(Serialize,Deserialize)]
pub struct NumberOfPackets {
    packet_length: u8,
}

pub enum PacketType {
    DataPacketStructure(DataPacketStructure),
    NumberOfPackets(NumberOfPackets),
}

pub fn create_data_packet(data: Vec<u8>) -> DataPacketStructure {
    DataPacketStructure{
        id: 0,
        seq_no: 0,
        data,
    }
}

pub fn type_match_packet(bytes: &[u8]) {
    println!("bytes {bytes:?}");
    // decode udp packet attributes
    // let d: DataPacketStructure = decode(bytes);

}

