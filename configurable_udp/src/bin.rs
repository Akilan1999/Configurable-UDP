//! # Udp Reliable
//!
//! this is a reliable udp protocol.

pub mod error;
pub mod reliable;
pub mod utils;

use crate::reliable::reliable::{create_data_packet, type_match_packet};
use crate::utils::{ende::encode, utils::init_udp};

/// Running example.
fn main() -> ! {
    let soc = init_udp("3245")
        .inspect(|v| {
            dbg!(v);
        })
        .inspect_err(|e| {
            dbg!(e);
        })
        .unwrap();
    let soc1 = init_udp("3246")
        .inspect(|v| {
            dbg!(v);
        })
        .inspect_err(|e| {
            dbg!(e);
        })
        .unwrap();

    let str: &str = "lol";
    let data = create_data_packet(str.as_bytes().to_vec());

    let mut encode_buf: [u8; 6555] = [0; 6555];

    let encoded_len = encode(&data, &mut encode_buf)
        .inspect(|encoded| {
            dbg!(encoded);
        })
        .inspect_err(|e| {
            dbg!(e);
        })
        .expect("Did not encode data");

    // Self test send random bytes
    soc.send_to(&encode_buf[..encoded_len], "0.0.0.0:3246")
        .inspect(|enc| {
            dbg!(enc);
        })
        .inspect_err(|e| {
            dbg!(e);
        })
        .expect("send_to should succeed");
    println!("RUNNING");

    // should be the same packet size (For testing fixed to 50 bytes)
    let mut buf = [0; 6555];

    // Test to see if both bytes are received
    // println!("received {received} bytes {:?}", &buf)
    loop {
        match soc1.recv(&mut buf) {
            Ok(received_len) => {
                println!("running");
                type_match_packet(
                    buf.get(..received_len)
                        .inspect(|received| {
                            dbg!(received);
                        })
                        .expect("expected to decode bincode"),
                )
            }
            Err(e) => println!("recv function failed: {e:?}"),
        }
    }
}
