//! usercalls module

use std::{io::Error, net::UdpSocket, thread};
use std::time::Duration;
use crate::reliable::reliable::{DataPacketStructure, packet_handling, Packet};
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
        time_before_resend: 3,
    };
    (init_udp(port),conf)
}

pub fn send<T: bincode::Encode>(soc: &UdpSocket,
                                data: T,
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

// TODO: Improve the error handling bits and is there a better way of writing loops
pub fn send_reliable<T: bincode::Encode>(soc: &UdpSocket,data: T,
                                         dest_addr: &str,
                                         transmission_id: u8,
                                         config: Config) {
    // Send for the first time
    // TODO: implement error handling if node not reachable
    send(soc,data,dest_addr,transmission_id);

    // Mutable variables only on the local scope of the
    // function. Should not create any deadlocks.
    let mut timer = config.time_before_resend;
    let mut retries:u64 = config.retries;

    loop {
        let data = recv(soc);
        let raw_data = data.unwrap();
        // If the length is 1 it's an packet
        if raw_data.len() == 1 && raw_data[0] == transmission_id {
            break
        }

        dbg!("sleep");
        // Sleep the loop for a millisecond
        thread::sleep(Duration::from_millis(1));
        timer -= 1;

        // Attempting to resend
        if timer == 0 && retries != 0 {
            dbg!("resending");
            send(soc,raw_data,dest_addr,transmission_id);
            retries -= 1;
            timer = config.time_before_resend;
        } else if retries == 0 {
            dbg!("no successful receive");
            break
        }
    }
}

pub fn recv(socket: &UdpSocket) -> Option<Vec<u8>> {
    let mut buf = [0; BUFFER_SIZE];

    match socket.recv_from(&mut buf) {
        Ok((received_len, send_addr)) => {
            let data = packet_handling(
                buf.get(..received_len).expect("expected to decode bincode"),
                send_addr,
                socket.try_clone().expect("soc1 clone failed"),
            );
            data
        },
        Err(e) => {
            println!("recv function failed: {e:?}");
            None
        },
    }
}


