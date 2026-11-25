//! usercalls module

use std::{io::Error, net::UdpSocket};
use std::time::Duration;
use crate::reliable::reliable::{DataPacketStructure, packet_handling};
use crate::utils::{encode_decode::encode, utils::init_udp};
use crate::utils::encode_decode::encode_vec;

// Temporary measure
const BUFFER_SIZE: usize = 6555;

pub struct Config {
    pub retries: u64,
    pub time_before_resend: u64, // in ms
}

pub fn init(port: &str) -> (Result<UdpSocket,Error>,Config) {
    // set as defaults
    let conf = Config {
        retries: 3,
        time_before_resend: 1,
    };
    (init_udp(port),conf)
}

pub fn send<T: bincode::Encode>(soc: &UdpSocket,
                                data: &T,
                                dest_addr: &str,
                                transmission_id: u8) {
    let vec_data = encode_vec(data).expect("Did not encode data");

    let data_struct = DataPacketStructure::create(vec_data,transmission_id);

    let mut encode_buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let enc_data = encode(&data_struct, &mut encode_buf)
        .expect("Did not encode data");

    soc.send_to(
        enc_data
            .get_slice()
            .expect("could not get data slice from encoded data"),
        dest_addr,
    ).expect("send_to should succeed");
}


pub fn send_reliable<T: bincode::Encode>(soc: &UdpSocket,data: T,
                                         dest_addr: &str,
                                         transmission_id: u8,
                                         config: Config) {

    send(soc,&data,dest_addr,transmission_id);

    // Mutable variables only on the local scope of the
    // function. Should not create any deadlocks.
    let mut retries:u64 = config.retries;

    loop {
        let data_recv = recv(soc,config.time_before_resend);
        if !data_recv.is_none() {
            let raw_data = data_recv.unwrap();
            // If the length is 1 it's an ack packet
            dbg!(raw_data[0]);
            if raw_data.len() == 1 && raw_data[0] == transmission_id {
                break
            }
        }
        // Attempting to resend
        if retries != 0 {
            send(soc,&data,dest_addr,transmission_id);
            retries -= 1;
        } else if retries == 0 {
            break
        }
    }
}

pub fn recv(socket: &UdpSocket, timeout: u64) -> Option<Vec<u8>> {
    let mut buf = [0; BUFFER_SIZE];

    socket.set_read_timeout(Some(Duration::from_millis(timeout))).expect("expected to kill the socket after a timeout");

    match socket.recv_from(&mut buf) {
        Ok((received_len, send_addr)) => {
            let data = packet_handling(
                buf.get(..received_len).expect("expected to decode bincode"),
                send_addr,
                socket.try_clone().expect("soc1 clone failed"),
            );
            data
        },
        Err(_) => {
            None
        },
    }
}


