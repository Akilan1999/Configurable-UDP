//! # Udp Reliable
//!
//! this is a reliable udp protocol.
pub mod reliable;
pub mod utils;
pub mod error;

use crate::reliable::reliable::{create_data_packet, type_match_packet};
use crate::utils::{bincode::encode, utils::init_udp};

/// Running example.
fn main() -> ! {
    let soc = init_udp("3245").unwrap();
    let soc1 = init_udp("3246").unwrap();

    let str: &'static str = "Hello world testing a random long string ";
    let data = create_data_packet(str.as_bytes().to_vec());

    let encoded = encode(data).expect("Did not encode data");
    let bytes: &[u8] = &encoded;

    println!("encoded bytes {bytes:?}");

    // Self test send random bytes
    soc.send_to(bytes, "0.0.0.0:3246")
        .expect("send_to should succeed");

    // should be the same packet size (For testing fixed to 50 bytes)
    let mut buf = [0; 655];

    // Test to see if both bytes are received
    // println!("received {received} bytes {:?}", &buf)
    loop {
        match soc1.recv(&mut buf) {
            Ok(received) => {
                type_match_packet(buf.get(..received).expect("expected to get buffer data"));
            }
            Err(e) => println!("recv function failed: {e:?}"),
        }
    }
}
