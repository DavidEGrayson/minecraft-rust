extern crate yaml_rust;
use std::net::TcpStream;
use std::io::prelude::*;
mod protocol;
use protocol::pack;
use protocol::packet;
mod settings;

trait EncodablePacket {
    fn encode(&self) -> Vec<u8>;
}

impl EncodablePacket for packet::Handshake {
    fn encode(&self) -> Vec<u8> {
        let mut r : Vec<u8> = Vec::new();
        r.extend(&pack::encode_varint(0x00));  // packet ID
        r.extend(&pack::encode_varint(self.protocol_version));
        r.extend(&pack::encode_string(&self.server_address));
        r.extend(&pack::encode_u16(self.server_port));
        r.extend(&pack::encode_varint(self.next_state as u32));
        pack::wrap_packet(r)
    }
}

impl EncodablePacket for packet::LoginStart {
    fn encode(&self) -> Vec<u8> {
        let mut r : Vec<u8> = Vec::new();
        r.extend(&pack::encode_varint(0x00));  // packet ID
        r.extend(&pack::encode_string(&self.username));
        pack::wrap_packet(r)
    }
}

fn print_bytes(bytes : &[u8]) {
    for byte in bytes {
        print!("0x{:02x} ", byte);
    }
    println!("");
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

    let handshake = packet::Handshake {
        protocol_version: 107,  // current protocol version
        server_address: server_address,
        server_port: server_port,
        next_state: 2, // login
    };
    let login_start = packet::LoginStart {
        username: username,
    };
    print_bytes(&handshake.encode());
    stream.write(&handshake.encode()).unwrap();
    print_bytes(&login_start.encode());
    stream.write(&login_start.encode()).unwrap();

    let packet = pack::read_packet(&mut stream).unwrap();
    println!("Packet:");
    print_bytes(&packet);

    let mut buffer : [u8; 100] = [0; 100];
    let byte_count = stream.read(&mut buffer).unwrap();
    println!("Left-over bytes:");
    print_bytes(&buffer[0..byte_count]);
}
