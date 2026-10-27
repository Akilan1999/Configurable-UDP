//! Reliability module.

use bincode::{Decode, Encode};
use crate::utils::queue::Queue;
use crate::utils::bincode::decode;
use std::sync::{LazyLock, Mutex};


// ------------------ Packet IDs -------------------
const DATA_PACKET_STRUCTURE_CODE: u64 = 1;
const NUMBER_OF_PACKETS_CODE: u64 = 2;
// -------------------------------------------------


/// Defines number of packets.
#[derive(Debug ,Encode, Decode)]
pub struct NumberOfPackets {
    /// concrete length.
    packet_length: u8,
    /// type of packet.
    packet_type: u64,
}

/// define a global result queue
/// Single threaded queue
pub static mut CUSTOMPACKET: LazyLock<Mutex<Queue<&[u8]>>> = LazyLock::new(|| Mutex::new(Queue::new()));


// ----------------------------- For extracting packet ID ---------------------------------
#[derive(Debug, Encode, Decode)]
pub struct DecodePacket {
    pub packet_type: u64,
}
// -------------------------------------------------------------------------------------


// ----------------------------- Data packets logic ---------------------------------
#[derive(Debug, Encode, Decode)]
pub struct DataPacketStructure {
    /// type of packet.
    packet_type: u64,
    /// sequence number.
    seq_no: u64,
    /// inner data.
    data: Vec<u8>,
}

/// Creates a Data packet from a vector of bytes.
pub fn create_data_packet(data: Vec<u8>) -> DataPacketStructure {
    DataPacketStructure {
        packet_type: DATA_PACKET_STRUCTURE_CODE,
        // Hard code: To be defined based on the fragmentation
        // logic
        seq_no: 2,
        data,
    }
}

/// The goal is to create an assembly queue for the data packets
/// Currently would store all packets in memory and then used defined location
/// In our case a shared queue for a user to pull from.
pub fn handle_data_packet(data: DataPacketStructure) {

}

// ------------------------------------------------------------------------------------

/// Matching on packet type.
pub fn type_match_packet(bytes: &[u8]) {

    // Decodes the type of packet intercepted
    let (packet_type,_): (DecodePacket,usize) = decode(bytes).expect("expected to parse data packet");

    match packet_type.packet_type {
        DATA_PACKET_STRUCTURE_CODE => {
            let (_data_packet,_size): (DataPacketStructure,usize) = decode(bytes).expect("expected to parse data packet");
            // TODO custom handlers
        },
        NUMBER_OF_PACKETS_CODE => {
            let (_data_packet,_size): (NumberOfPackets,usize) = decode(bytes).expect("expected to parse data packet");
            // TODO custom handlers
        },
        _ => unsafe {
            // For handling custom packets passed through
            // CUSTOMPACKET.lock().unwrap().enqueue(bytes)
        }
    }
}

