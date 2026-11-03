//! usercalls module

use std::net::UdpSocket;

pub fn init() {
    // Creates a UDP socket with a random port no:
    // - The logic for the assembler as well
    // - Packet configurations
}

// For initial testing reasons we will assume
// that all structs do not have to be
// broken down into a fragment and
// can be sent over directly.
pub fn send<T>(data: T, dest_addr: &str) {
    
}
pub fn recv<T>(soc: UdpSocket) {

}


