//! # Udp Reliable
//!
//! this is a reliable udp protocol.

pub mod error;
pub mod reliable;
pub mod utils;

use crate::reliable::reliable::{DataPacketStructure, create_data_packet, type_match_packet_serde};
use crate::utils::bincode::decode;
use crate::utils::{bincode::encode, utils::init_udp};

/// Running example.
fn main() -> ! {
    let soc = init_udp("3245").unwrap();
    let soc1 = init_udp("3246").unwrap();

    let str: &str = "lol";
    let data = create_data_packet(str.as_bytes().to_vec());

    let encoded = encode(&data).expect("Did not encode data");

    // Self test send random bytes
    soc.send_to(&encoded, "0.0.0.0:3246")
        .expect("send_to should succeed");

    // should be the same packet size (For testing fixed to 50 bytes)
    let mut buf = [0; 6555];

    // Test to see if both bytes are received
    // println!("received {received} bytes {:?}", &buf)
    loop {
        match soc1.recv(&mut buf) {
            Ok(received) => {
                let (data_packet, len): (DataPacketStructure, usize) =
                    decode(buf.get(..received).expect("expected to decode bincode"))
                        .expect("expected to parse data packet");
                dbg!(data_packet);
                // type_match_packet_serde(buf.get(..received).expect("expected to get buffer data"));
            }
            Err(e) => println!("recv function failed: {e:?}"),
        }
    }
}
