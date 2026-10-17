use crate::utils::utils::*;

pub mod utils;

fn main() {
    let soc = init_udp("3245".parse().unwrap());
    let soc1 = init_udp("3246".parse().unwrap());
    // std::thread::spawn(|| {
    //     listen();
    // });

    // Self test send random bytes
    soc.send_to(&[0; 3],"0.0.0.0:3246").expect("send_to should succeed");
    soc.send_to(&[0; 5],"0.0.0.0:3246").expect("send_to should succeed");

    let mut buf = [0; 10];

    // Test to see if both bytes are received
    loop {
        match soc1.recv(&mut buf) {
            Ok(received) => println!("received {received} bytes {:?}", &buf[..received]),
            Err(e) => println!("recv function failed: {e:?}"),
        }
    }

}