use crate::utils::packet_structure::{create_data_packet, encode_packet, type_match_packet};
use crate::utils::utils::*;

pub mod utils;

fn main() {
    let soc = init_udp("3245".parse().unwrap());
    let soc1 = init_udp("3246".parse().unwrap());


    let foo = "Hello world testing a random long string ";
    let data = create_data_packet(foo.as_bytes().to_vec());

    let encoded = encode_packet(data);
    let bytes: &[u8] = &encoded;

    println!("encoded bytes {:?}", &bytes);

    // Self test send random bytes
    soc.send_to(bytes,"0.0.0.0:3246").expect("send_to should succeed");

    // should be the same packet size (For testing fixed to 50 bytes)
    let mut buf = [0; 655];

    // Test to see if both bytes are received
    loop {
        match soc1.recv(&mut buf) {
            Ok(received) => println!("received {received} bytes {:?}", &buf),
            Err(e) => println!("recv function failed: {e:?}"),
        }
    }

}