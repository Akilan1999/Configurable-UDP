//! Reliability module.

use crate::utils::encode_decode::{decode, encode};
use bincode::{Decode, Encode};
use std::net::{SocketAddr, UdpSocket};

// ------------------ Packet IDs -------------------
const DATA_PACKET_STRUCTURE_CODE: u64 = 1;
const NUMBER_OF_PACKETS_CODE: u64 = 2;
const PACKETS_REQUIRED: u64 = 3;
// --------------------------------------------------
// -------------- Common attributes ----------------
pub trait Packet {
    fn process(self, socket :SocketInformation);
}

// ------------------------------------------------

/// Defines number of packets.
#[derive(Debug, Copy, Clone, Encode, Decode)]
pub struct NumberOfPackets {
    /// type of packet.
    packet_type: u64,
    /// packet.
    number_of_packets: u8,
    /// chuck size of each of the packets.
    packet_chuck_size: u8,
}

/// define a global result queue
/// Single threaded queue.
// pub static mut CUSTOMPACKET: LazyLock<Mutex<Queue<&[u8]>>> =
//     LazyLock::new(|| Mutex::new(Queue::new()));

// ----------------------------- For extracting packet ID ---------------------------------
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct DecodePacket {
    pub packet_type: u64,
}
// -------------------------------------------------------------------------------------
// ----------------------------- Socket information -----------------------------------

#[derive(Debug)]
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

impl DataPacketStructure {
    pub(crate) fn create(data: Vec<u8>) -> Self {
        DataPacketStructure {
            packet_type: DATA_PACKET_STRUCTURE_CODE,
            // Hard code: To be defined based on the fragmentation
            // logic
            seq_no: 2,
            data,
        }

    }
}

impl Packet for DataPacketStructure {
    fn process(self,socket :SocketInformation) {
        dbg!(String::from_utf8_lossy(&self.data));
    }
}

/// Creates a Data packet from a vector of bytes.
/// TODO: Implement this later on.
// impl<T> From<T> for DataPacketStructure
// where
//     T: Into<Vec<u8>>,
// {
//     fn from(into_bytes: T) -> Self {
//         DataPacketStructure {
//             packet_type: DATA_PACKET_STRUCTURE_CODE,
//             // Hard code: To be defined based on the fragmentation
//             // logic
//             seq_no: 2,
//             data: into_bytes.into(),
//         }
//     }
// }

/// The goal is to create an assembly queue for the data packets
/// Currently would store all packets in memory and then used defined location
/// In our case a shared queue for a user to pull from.
// pub fn handle_data_packet(data: DataPacketStructure, socket_info: SocketInformation) {
//     // Sends an ACK packet of the last packet received
//     // send_ack(data.seq_no,socket_info)
// }
// ---------------------------------- Send ACK ----------------------------------------
/// This is for temporary measure for testing. The goal is to not have any ACK and NAK packets.
/// Rather the receiver after a certain interval for retransmission of the missing packets.
/// This allows reliability of packets in any order and programmer decides at a higher level
/// The sliding window (https://en.wikipedia.org/wiki/Sliding_window_protocol).

#[derive(Debug, Encode, Decode)]
pub struct PacketRequired {
    /// type of packet.
    packet_type: u64,
    /// type of packet.
    packets_required: Vec<u8>,
}
pub fn send_ack(packets_required: Vec<u8>, socket_info: SocketInformation) {
    // Create Ack packet
    let packets_required = PacketRequired {
        packet_type: PACKETS_REQUIRED,
        packets_required,
    };
    // ------------- Encode Ack packet and transmit it through ------------
    let mut encode_buf: [u8; 6555] = [0; 6555];
    let enc_data = encode(&packets_required, &mut encode_buf).expect("Did not encode data");
    socket_info
        .socket
        .send_to(
            enc_data
                .get_slice()
                .expect("could not get slice from encoded data"),
            socket_info.send_addr,
        )
        .expect("send_to should succeed");
    // --------------------------------------------------------------------
}

pub fn recv_ack(socket_info: SocketInformation) {}
// ------------------------------------------------------------------------------------

/// Matching on packet type.
pub fn type_match_packet(bytes: &[u8], source_addr: SocketAddr, socket: UdpSocket) {
    // Decodes the type of packet intercepted
    let packet_type: DecodePacket = decode(bytes).expect("expected to parse data packet");

    // Create the socker information
    let socket_info = SocketInformation {
        socket,
        send_addr: source_addr,
    };

    match packet_type.packet_type {
        DATA_PACKET_STRUCTURE_CODE => {
            let data_packet: DataPacketStructure =
                decode(bytes).expect("expected to parse data packet");
            DataPacketStructure::process(data_packet, socket_info);
        }
        NUMBER_OF_PACKETS_CODE => {
            let num_packet: NumberOfPackets = decode(bytes).expect("expected to parse data packet");
            dbg!(&num_packet);
        }
        PACKETS_REQUIRED => {
            let ack: PacketRequired = decode(bytes).expect("expected to parse data packet");
            dbg!(&ack);
        }
        _ => {
            // For handling custom packets passed through
            // CUSTOMPACKET.lock().unwrap().enqueue(bytes)
        }
    }
}
