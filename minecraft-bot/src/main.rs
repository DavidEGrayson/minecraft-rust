extern crate yaml_rust;
use std::net::TcpStream;
use std::io::prelude::*;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

mod protocol;
use protocol::pack;
use protocol::packet;
use protocol::encode::EncodablePacket;
mod settings;
mod util;

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
        // TODO: this needs to be improved a lot
        match *self {
            LoginError::RefusedByServer(ref s) => {
                s
            }
            LoginError::IOError(ref e) => { "I/O error" }
        }
    }
}

type LoginResult = Result<(), LoginError>;

fn login(stream : &mut TcpStream, server_address : &str,
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
    stream.write(&handshake.encode()).unwrap();
    util::print_bytes(&handshake.encode());
    stream.write(&login_start.encode()).unwrap();
    util::print_bytes(&login_start.encode());

    println!("Reading packet...");
    let raw_packet = match pack::read_packet(stream) {
        Ok(p) => p,
        Err(e) => { panic!(); }
    };
    println!("Raw packet:");
    util::print_bytes(&raw_packet);
    let packet = protocol::decode::decode(raw_packet);
    match &packet {
        &packet::Packet::Disconnect(ref d) => println!("Disconnected from server: {}", d.reason),
        _ => println!("Got some other kind of packet"),
    }
    println!("Processed packet: {:?}", packet);

    let mut buffer : [u8; 100] = [0; 100];
    let byte_count = stream.read(&mut buffer).unwrap();
    println!("Left-over bytes:");
    util::print_bytes(&buffer[0..byte_count]);
    return Result::Ok(());
}

fn main() {
    let settings = settings::read();
    let server_address : String = settings["server"].as_str().unwrap().to_owned();
    let server_port : u16 = settings["server_port"].as_i64().unwrap() as u16;
    let server_name : String = server_address.to_owned() + ":" + &server_port.to_string();
    let username : String = settings["username"].as_str().unwrap().to_owned();

    println!("Connecting to {}...", server_name);
    let mut stream = TcpStream::connect(&*server_name).unwrap();
    println!("Connected.");

    match login(&mut stream, &server_address, server_port, &username) {
        Result::Ok(_) => (),
        Result::Err(e) => {
            println!("Failed to log in: {}", e);
            return;
        }
    };

    println!("Successfully logged in.");
}
