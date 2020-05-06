//! Creates and maintain a simple NREPL server.

use std::{
    dbg,
    io::{Read, Result},
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    str,
};

const SERVER_PORT: u16 = 5555;

/// Returns the address of the server.
fn get_address(port: u16) -> SocketAddrV4 {
    let lh = Ipv4Addr::LOCALHOST;
    SocketAddrV4::new(lh, port)
}

/// TODO: add a nice way to change the server port instead of always using the
/// default one.
fn nrepl_default_address() -> SocketAddrV4 {
    get_address(SERVER_PORT)
}

pub struct Server;

impl Server {
    /// Creates a new server and runs it.
    pub fn run() -> Result<()> {
        let addr = nrepl_default_address();
        let listener = TcpListener::bind(addr)?;

        println!("Server started");
        let (mut stream, addr) = listener.accept()?;
        println!("New client at {}", addr);

        loop {
            let mut input_buffer = [0; 128];
            stream.read(&mut input_buffer);

            if input_buffer[0] == 0 {
                break;
            }

            let s = str::from_utf8(&input_buffer).unwrap();
            println!("Got data from {}: {}", &addr, s);
        }

        Ok(())
    }
}
