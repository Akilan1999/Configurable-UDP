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
use crate::usercalls::usercalls::{init, recv, send};

trait Handler {
    fn handle(self,socket: UdpSocket);
}

impl Handler for test {
    fn handle(self,socket: UdpSocket) {
        todo!()
    }
}

#[derive(Debug, Encode, Decode)]
struct test {
    name: String,
    number: u8
}

/// Running example.
fn main() -> ! {
    let soc2 = init("3245").unwrap();
    let soc1 = init("3246").unwrap();

    let a = test {
        number: 105,
        name: String::from("Test lol"),
    };
    send(&soc2,a,"0.0.0.0:3246");

    loop {
        // Socket 1 receiving data
        recv(&soc1);
        // Socket 2 receiving data
        recv(&soc2);
    }
}
