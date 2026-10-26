//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! # Udp Reliable
//!
//! this is a reliable udp

use crate::utils::bincode::{*};
use crate::reliable::reliable::{*};
use crate::utils::utils::*;

pub mod utils;
pub mod reliable;

/// Running example
fn main() {
    let soc = init_udp("3245".parse().unwrap());
    let soc1 = init_udp("3246".parse().unwrap());


    let foo = "lol";
    let data = create_data_packet(foo.as_bytes().to_vec());

    let encoded = encode(&data);

    // Self test send random bytes
    soc.send_to(&encoded,"0.0.0.0:3246").expect("send_to should succeed");

    // should be the same packet size (For testing fixed to 50 bytes)
    let mut buf = [0; 6555];

    // Test to see if both bytes are received
    // println!("received {received} bytes {:?}", &buf)
    loop {
        match soc1.recv(&mut buf) {
            Ok(received) => type_match_packet(&buf[..received]),
            Err(e) => println!("recv function failed: {e:?}"),
        }
    }

}
