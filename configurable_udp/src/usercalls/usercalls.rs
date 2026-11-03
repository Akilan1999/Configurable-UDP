//! usercalls module

use std::{io::Error, net::UdpSocket};
use std::net::SocketAddr;
use bincode::encode_to_vec;
use crate::reliable::reliable::{DataPacketStructure, type_match_packet, Packet};
use crate::utils::{encode_decode::encode, utils::init_udp};
use crate::utils::encode_decode::encode_vec;


// Temporary measure
const BUFFER_SIZE: usize = 6555;

pub fn init(port: &str) -> Result<UdpSocket,Error> {
    init_udp(port)
}

// For initial testing reasons we will assume
// that all structs do not have to be
// broken down into a fragment and
// can be sent over directly.
// TODO: Result -> T to be replaced with standard transmission related information
pub fn send<T: bincode::Encode>(soc: &UdpSocket, data: T, dest_addr: &str) {
    // ------------------------------------------------------------------------
    let vec_data = encode_vec(data).inspect(|encoded| {
        dbg!(encoded);
    })
        .inspect_err(|e| {
            dbg!(e);
        })
        .expect("Did not encode data");
    // ------------------------------------------------------------------------
    let data_struct = DataPacketStructure::create(vec_data);
    let mut encode_buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    // ------------------------------------------------------------------------
    let enc_data = encode(&data_struct, &mut encode_buf)
        .inspect(|encoded| {
            dbg!(encoded);
        })
        .inspect_err(|e| {
            dbg!(e);
        })
        .expect("Did not encode data");
    // ------------------------------------------------------------------------
    // ------------------------------------------------------------------------
    soc.send_to(
        enc_data
            .get_slice()
            .expect("could not get data slice from encoded data"),
        dest_addr,
    )
        .inspect(|enc| {
            dbg!(enc);
        })
        .inspect_err(|e| {
            dbg!(e);
        })
        .expect("send_to should succeed");
    // ------------------------------------------------------------------------
}
pub fn recv(socket: &UdpSocket) {
    let mut buf = [0; BUFFER_SIZE];

    match socket.recv_from(&mut buf) {
        Ok((received_len, send_addr)) => type_match_packet(
            buf.get(..received_len).expect("expected to decode bincode"),
            send_addr,
            socket.try_clone().expect("soc1 clone failed"),
        ),
        Err(e) => println!("recv function failed: {e:?}"),
    }
}


