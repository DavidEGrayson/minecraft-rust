extern crate yaml_rust;
use std::net::TcpStream;
use std::io::prelude::*;
mod protocol;
use protocol::pack;
use protocol::packet;
use protocol::encode::EncodablePacket;
mod settings;
mod util;

fn main() {
    let settings = settings::read();
    let server_address : String = settings["server"].as_str().unwrap().to_owned();
    let server_port : u16 = settings["server_port"].as_i64().unwrap() as u16;
    let server_name : String = server_address.to_owned() + ":" + &server_port.to_string();
    let username : String = settings["username"].as_str().unwrap().to_owned();

    println!("Connecting to {}...", server_name);
    let mut stream = TcpStream::connect(&*server_name).unwrap();
    println!("Connected.");

    let handshake = packet::Handshake {
        protocol_version: 107,  // current protocol version
        server_address: server_address,
        server_port: server_port,
        next_state: 2, // login
    };
    let login_start = packet::LoginStart {
        username: username,
    };
    util::print_bytes(&handshake.encode());
    stream.write(&handshake.encode()).unwrap();
    util::print_bytes(&login_start.encode());
    stream.write(&login_start.encode()).unwrap();

    let raw_packet = pack::read_packet(&mut stream).unwrap();
    println!("Raw packet:");
    util::print_bytes(&raw_packet);
    let packet = protocol::decode::decode(raw_packet);
    match &packet {
        &packet::Packet::Disconnect(ref d) => println!("Disconnected from server: {}", d.reason),
        _ => (),
    }
    println!("Processed packet: {:?}", packet);

    let mut buffer : [u8; 100] = [0; 100];
    let byte_count = stream.read(&mut buffer).unwrap();
    println!("Left-over bytes:");
    util::print_bytes(&buffer[0..byte_count]);
}
