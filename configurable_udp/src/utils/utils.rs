//! Utils module
//! reference: <https://github.com/wangmarkqi/udp_hole_punching>.
use std::net::UdpSocket;

/// # initiating udp
/// this initiates udp.
pub(crate) fn init_udp(port: &str) -> Result<UdpSocket, std::io::Error> {
    let addr = format!("{}:{}", "0.0.0.0", port);

    UdpSocket::bind(addr)
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
