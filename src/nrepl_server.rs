//! Creates and maintain a simple NREPL server.

use std::{
    dbg,
    io::{BufReader, Read, Write, Result},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    str,
};

use crate::repl::Repl;

const SERVER_PORT: u16 = 5555;

/// Returns the address of the server.
fn get_address(port: u16) -> SocketAddrV4 {
    let lh = Ipv4Addr::LOCALHOST;
    SocketAddrV4::new(lh, port)
}

// TODO: add a nice way to change the server port instead of always using the
// default one.
fn nrepl_default_address() -> SocketAddrV4 {
    get_address(SERVER_PORT)
}

/// Returns whether if the tcp stream is ended or not.
fn is_empty(stream: &TcpStream) -> Result<bool> {
    let mut peeked_data = [0; 1];
    stream.peek(&mut peeked_data)?;
    let is_empty = peeked_data[0] == 0;
    Ok(is_empty)
}

pub struct Server;

impl Server {
    /// Creates a new server and runs it.
    pub fn run() -> Result<()> {
        let addr = nrepl_default_address();
        let listener = TcpListener::bind(addr)?;

        let (mut stream, addr) = listener.accept()?;

        println!("Starting server...");

        loop {
            if is_empty(&stream)? {
                break;
            }

            let mut buffer = [0; 128];
            let bytes_received = stream.read(&mut buffer)?;

            let s = str::from_utf8(&buffer).unwrap();
            println!("Received some data: {}", s);

            let to_send = "{:out \";; nREPL 0.6.0\n;; Clojure 1.10.0\nuser=> \"}";
            stream.write(to_send.as_bytes());

        }

        Ok(())
    }
}