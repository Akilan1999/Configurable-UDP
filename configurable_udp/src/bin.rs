//! # Udp Reliable
//!
//! this is a reliable udp protocol.

pub mod error;
pub mod reliable;
pub mod utils;
pub mod usercalls;

use std::net::UdpSocket;
use bincode::{Decode, Encode};
use crate::reliable::reliable::{Packet};
use crate::usercalls::usercalls::{init, recv, send, send_reliable};
use crate::utils::encode_decode::decode;

trait Handler {
    fn handle(self,socket: UdpSocket);
}

impl Handler for Test {
    fn handle(self,socket: UdpSocket) {
        todo!()
    }
}

#[derive(Debug, Encode, Decode)]
struct Test {
    name: String,
    number: u8
}

/// Running example.
fn main() {
    let soc2 = init("3245");
    let soc1 = init("3246");

    let soc2_net = soc2.0.unwrap();
    let soc1_net = soc1.0.unwrap();

    let a = Test {
        number: 105,
        name: String::from("Test lol"),
    };

    send_reliable(&soc2_net, a, "0.0.0.0:3246", 1, soc2.1);

    // send(&soc2_net,a,"0.0.0.0:3246",1);
    //
    // // Assume one packet at a time as a start
    loop {
        // Socket 1 receiving data
        let data = recv(&soc1_net);
        let d: Test = decode(data.unwrap()).unwrap();
        dbg!(d);
        // Match statement to cast it to the right struct
    }
}
