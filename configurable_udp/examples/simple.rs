//! # Udp Reliable
//!
//! this is a reliable udp protocol.

use std::net::UdpSocket;
use std::thread;
use bincode::{Decode, Encode};
use udp_configurable_lib::usercalls::usercalls::{init, recv, send, send_reliable};
use udp_configurable_lib::utils::encode_decode::decode;

trait Handler {
    fn handle(self,socket: UdpSocket);
}

impl Handler for Test {
    fn handle(self,socket: UdpSocket) {
        // todo work and send over
    }
}

#[derive(Debug, Encode, Decode)]
struct Test {
    name: String,
    number: u8
}

/// Running examples.
fn main() {
    let soc2 = init("3245");
    let soc1 = init("3246");

    let soc2_net = soc2.0.unwrap();
    let soc1_net = soc1.0.unwrap();

    let a = Test {
        number: 105,
        name: String::from("Test lol"),
    };

    // Sends the tasks as a background process
    thread::spawn(move || {
        send_reliable(&soc2_net, a, "0.0.0.0:3246", 10, soc2.1);
    });

    loop {
        // Socket 1 receiving data
        let data = recv(&soc1_net,soc1.1.time_before_resend);
        if !data.is_none() {
            let d: Test = decode(data.unwrap()).unwrap();
            dbg!(d);
        }
    }
}
