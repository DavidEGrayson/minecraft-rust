extern crate std;
use std::io::prelude::*;
use std::error::Error;
use std::convert::From;
use std::fmt::Display;
use std::fmt::Formatter;
use std::net::TcpStream;

use protocol;
use protocol::pack;
use protocol::packet;
use protocol::login;
use protocol::encode::EncodablePacket;
use util;

#[derive(Debug)]
pub enum LoginError {
    RefusedByServer(String),
    IOError(std::io::Error),
}

impl Display for LoginError {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match *self {
            LoginError::RefusedByServer(ref s) => {
                try!(fmt.write_str("Login refused by server"));
                try!(fmt.write_str(s));
                std::result::Result::Ok(())
            },
            LoginError::IOError(ref e) => { e.fmt(fmt) }
        }
    }
}

impl Error for LoginError {
    fn description(&self) -> &str {
        // TODO: this needs to be improved a lot if it is actually used
        match *self {
            LoginError::RefusedByServer(ref s) => { "login refused by server" }
            LoginError::IOError(ref e) => { "I/O error during login" }
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            LoginError::RefusedByServer(..) => Option::None,
            LoginError::IOError(ref e) => Option::Some(e),
        }
    }
}

impl From<std::io::Error> for LoginError {
    fn from(err : std::io::Error) -> LoginError {
        LoginError::IOError(err)
    }
}

pub type LoginResult = Result<(), LoginError>;

pub fn login(stream : &mut TcpStream, server_address : &str,
         server_port : u16, username : &str) -> LoginResult {
    let handshake = packet::Handshake {
        protocol_version: 109,  // current protocol version
        server_address: server_address.to_owned(),
        server_port: server_port,
        next_state: 2, // login
    };
    let login_start = packet::LoginStart {
        username: username.to_owned(),
    };
    try!(stream.write(&handshake.encode()));
    try!(stream.write(&login_start.encode()));

    println!("Reading packet...");
    let raw_packet = try!(pack::read_packet(stream));
    println!("Raw packet:");
    util::print_bytes(&raw_packet);
    let packet = protocol::decode::decode(raw_packet);
    match &packet {
        &packet::Packet::Disconnect(ref d) => {
            return Err(LoginError::RefusedByServer(d.reason.to_owned()));
        }
        _ => println!("Got some other kind of packet"),
    }
    println!("Processed packet: {:?}", packet);

    let mut buffer : [u8; 100] = [0; 100];
    let byte_count = stream.read(&mut buffer).unwrap();
    println!("Left-over bytes:");
    util::print_bytes(&buffer[0..byte_count]);
    return Result::Ok(());
}
