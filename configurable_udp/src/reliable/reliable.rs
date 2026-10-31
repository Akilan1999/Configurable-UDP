//! Reliability module.

use crate::utils::ende::{decode, encode};
use crate::utils::queue::Queue;
use bincode::{Decode, Encode};
use std::sync::{LazyLock, Mutex};
use std::net::{SocketAddr, UdpSocket};

// ------------------ Packet IDs -------------------
const DATA_PACKET_STRUCTURE_CODE: u64 = 1;
const NUMBER_OF_PACKETS_CODE: u64 = 2;
const ACK_CODE: u64 = 3;
// -------------------------------------------------

/// Defines number of packets.
#[derive(Debug, Encode, Decode)]
pub struct NumberOfPackets {
    /// concrete length.
    packet_length: u8,
    /// type of packet.
    packet_type: u64,
}

/// define a global result queue
/// Single threaded queue
// pub static mut CUSTOMPACKET: LazyLock<Mutex<Queue<&[u8]>>> =
//     LazyLock::new(|| Mutex::new(Queue::new()));

// ----------------------------- For extracting packet ID ---------------------------------
#[derive(Debug, Encode, Decode)]
pub struct DecodePacket {
    pub packet_type: u64,
}
// -------------------------------------------------------------------------------------
// ----------------------------- Socket information -----------------------------------
pub struct SocketInformation {
    pub socket: UdpSocket,
    pub send_addr: SocketAddr,
}
//-----------------------------------------------------------------------------------
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
pub fn handle_data_packet(data: DataPacketStructure, socket_info: SocketInformation) {
    // Sends an ACK packet of the last packet received
    send_ack(data.seq_no,socket_info)
}
// ---------------------------------- Send ACK ----------------------------------------
/// This is for temporary measure for testing. The goal is to not have any ACK and NAK packets.
/// Rather the receiver after a certain interval for retransmission of the missing packets.
/// This allows reliability of packets in any order and programmer decides at a higher level
/// The sliding window (https://en.wikipedia.org/wiki/Sliding_window_protocol).

#[derive(Debug, Encode, Decode)]
pub struct Ack {
    /// type of packet.
    packet_type: u64,
    /// type of packet.
    last_packet: u64,
}
pub fn send_ack(last_packet: u64, socket_info: SocketInformation) {
    // Create Ack packet
    let ack = Ack {
        packet_type: ACK_CODE,
        last_packet,
    };
    // ------------- Encode Ack packet and transmit it through ------------
    let mut encode_buf: [u8; 6555] = [0; 6555];
    let encoded_len = encode(&ack, &mut encode_buf)
        .expect("Did not encode data");
    socket_info.socket.send_to(&encode_buf[..encoded_len], socket_info.send_addr)
        .expect("send_to should succeed");
    // --------------------------------------------------------------------
}

pub fn recv_ack(socket_info: SocketInformation) {

}
// ------------------------------------------------------------------------------------

/// Matching on packet type.
pub fn type_match_packet(bytes: &[u8], source_addr: SocketAddr, socket: UdpSocket) {
    // Decodes the type of packet intercepted
    let (packet_type, _): (DecodePacket, usize) =
        decode(bytes).expect("expected to parse data packet");

    // Create the socker information
    let socket_info = SocketInformation {
        socket,
        send_addr: source_addr,
    };

    match packet_type.packet_type {
        DATA_PACKET_STRUCTURE_CODE => {
            let (data_packet, _): (DataPacketStructure, usize) =
                decode(bytes).expect("expected to parse data packet");
            dbg!(&data_packet);
            handle_data_packet(data_packet,socket_info);
        }
        NUMBER_OF_PACKETS_CODE => {
            let (num_packet, _): (NumberOfPackets, usize) =
                decode(bytes).expect("expected to parse data packet");
            dbg!(&num_packet);
            // TODO custom handlers
        }
        ACK_CODE => {
            let (ack, _): (Ack, usize) =
                decode(bytes).expect("expected to parse data packet");
            dbg!(&ack);
        }
        _ => {
            // For handling custom packets passed through
            // CUSTOMPACKET.lock().unwrap().enqueue(bytes)
        },
    }
}
