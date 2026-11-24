//! Reliability module.

use crate::utils::encode_decode::{decode, encode};
use bincode::{Decode, Encode};
use std::net::{SocketAddr, UdpSocket};

// ------------------------------ Packet IDs -----------------------------
const DATA_PACKET_STRUCTURE_CODE: u64 = 1;
const ACK: u64 = 2;
// -----------------------------------------------------------------------
// -------------------------- General attributes --------------------------
pub trait Packet {
    fn process(self, socket :SocketInformation) -> Option<Vec<u8>>;
}

// for extracting packet ID
#[derive(Debug, Encode, Decode)]
pub struct DecodePacket {
    pub packet_type: u64,
}

// ----------------------------------------------------------------------------
// ----------------------------- Socket information ---------------------------

#[derive(Debug)]
pub struct SocketInformation {
    pub socket: UdpSocket,
    pub send_addr: SocketAddr,
}
//--------------------------------------------------------------------------------
// ----------------------------- Data packets logic ------------------------------
#[derive(Debug, Encode, Decode)]
pub struct DataPacketStructure {
    /// type of packet.
    packet_type: u64,
    /// transmission id
    transmission_id: u8,
    /// inner data.
    data: Vec<u8>,
}

impl DataPacketStructure {
    pub(crate) fn create(data: Vec<u8>,transmission_id: u8) -> Self {
        DataPacketStructure {
            packet_type: DATA_PACKET_STRUCTURE_CODE,
            transmission_id,
            data,
        }

    }
}

impl Packet for DataPacketStructure {
    fn process(self,socket :SocketInformation) -> Option<Vec<u8>> {
        // Send ack packet
        send_ack(self.transmission_id, socket);
        Some(self.data)
    }
}

//--------------------------------------------------------------------------------
// ----------------------------- Ack packets logic -------------------------------

#[derive(Debug, Encode, Decode)]
pub struct Ack {
    /// type of packet.
    packet_type: u64,
    transmission_id: u8,
}
pub fn send_ack(transmission_id: u8, socket_info: SocketInformation) {
    // Create Ack packet
    let packets_required = Ack {
        packet_type: ACK,
        transmission_id,
    };
    // Encode Ack packet and transmit it through
    let mut encode_buf: [u8; 6555] = [0; 6555];

    let enc_data = encode(&packets_required, &mut encode_buf)
        .expect("Did not encode data");

    socket_info
        .socket
        .send_to(
            enc_data
                .get_slice()
                .expect("could not get slice from encoded data"),
            socket_info.send_addr,
        )
        .expect("send_to should succeed");
}
 impl Packet for Ack {
     fn process(self, socket :SocketInformation) -> Option<Vec<u8>> {
         Some(vec![self.transmission_id])
     }
 }


//--------------------------------------------------------------------------------
// ----------------------------- General packet logic ----------------------------

pub enum Packets {
    Data(DataPacketStructure),
    Ack(Ack),
}

impl Packet for Packets {
    fn process(self, socket: SocketInformation) -> Option<Vec<u8>> {
        match self {
            Packets::Data(data_packet_structure) => {
                data_packet_structure.process(socket)
            },
            Packets::Ack(ack) => {
                ack.process(socket)
            }
        }
    }
}
pub fn decode_prime(bytes: &[u8]) -> Option<Packets> {
    let packet_type: DecodePacket = decode(bytes).expect("expected to parse data packet");

    match packet_type.packet_type {
        DATA_PACKET_STRUCTURE_CODE => {
            let data_packet: DataPacketStructure =
                decode(bytes).expect("expected to parse data packet");
            Some(Packets::Data(data_packet))
        }
        ACK => {
            let ack: Ack = decode(bytes).expect("expected to parse data packet");
            Some(Packets::Ack(ack))
        }
        _ => None
    }
}

pub fn packet_handling(bytes: &[u8], source_addr: SocketAddr, socket: UdpSocket) -> Option<Vec<u8>> {

    let packet_type: Packets = decode_prime(bytes)
        .expect("expected to parse data packet");

    // Create the socker information
    let socket_info = SocketInformation {
        socket,
        send_addr: source_addr,
    };

    packet_type.process(socket_info)
}
//-----------------------------------------------------------------------------------
