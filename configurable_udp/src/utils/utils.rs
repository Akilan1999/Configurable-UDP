//! Utils module


//! reference: <https://github.com/wangmarkqi/udp_hole_punching>
use std::net::UdpSocket;

/// # initiating udp
/// this initiates udp
pub fn init_udp(port: String) -> UdpSocket {
    let addr = format!("{}:{}", "0.0.0.0", port);
    let soc = UdpSocket::bind(addr).expect("udp can not open");
    soc
}

// Future
// pub fn listen() {
//     let res = block_on(async {
//         _listen().await
//     });
//     match res {
//         Ok(()) => dbg!("everything ok"),
//         Err(e) => dbg!(&e.to_string()),
//     };
// }
//
// async fn _listen() -> anyhow::Result<()> {
//     loop {
//         println!("test")
//     }
// }
