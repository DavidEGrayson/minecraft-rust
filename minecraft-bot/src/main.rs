extern crate yaml_rust;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;
use yaml_rust::YamlLoader;

fn encode_varint(n : u32) -> Vec<u8> {
    if n > 127 {
        let mut r : Vec<u8> = vec![(n as u8) | 0x80];
        r.extend(&encode_varint(n >> 7));
        r
    }
    else {
        return vec![n as u8];
    }
}

fn encode_u16(n : u16) -> Vec<u8> {
    return vec![(n >> 8) as u8, n as u8];
}

fn encode_string(s : &str) -> Vec<u8>
{
    let mut r : Vec<u8> = encode_varint(s.len() as u32); // TODO: don't use 'as' here
    r.extend(s.to_owned().into_bytes());
    r
}

trait EncodablePacket {
    fn encode(&self) -> Vec<u8>;
}

fn wrap_packet(x : Vec<u8>) -> Vec<u8>
{
    let mut r = encode_varint(x.len() as u32);
    r.extend(x);
    r
}

struct Handshake {
    protocol_version : u32,
    server_address : String,
    server_port: u16,
    next_state : u32,  // 1 for status, 2 for login
}

impl EncodablePacket for Handshake {
    fn encode(&self) -> Vec<u8> {
        let mut r : Vec<u8> = Vec::new();
        r.extend(&encode_varint(0x00));  // packet ID
        r.extend(&encode_varint(self.protocol_version));
        r.extend(&encode_string(&self.server_address));
        r.extend(&encode_u16(self.server_port));
        r.extend(&encode_varint(self.next_state as u32));
        wrap_packet(r)
    }
}

struct LoginStart {
    username : String
}

impl EncodablePacket for LoginStart {
    fn encode(&self) -> Vec<u8> {
        let mut r : Vec<u8> = Vec::new();
        r.extend(&encode_varint(0x00));  // packet ID
        r.extend(&encode_string(&self.username));
        wrap_packet(r)
    }
}

fn read_settings() -> yaml_rust::Yaml {
    let mut s = String::new();
    let mut f = File::open("config.yml").expect("open config file");
    f.read_to_string(&mut s).expect("read config file");
    let settings_docs = YamlLoader::load_from_str(&s).unwrap();
    if settings_docs.len() != 1
    {
        panic!("Settings file has wrong number of YAML documents, expected 1.");
    }
    assert_eq!(settings_docs.len(), 1);
    settings_docs[0].to_owned()
}

fn print_bytes(bytes : &[u8]) {
    for byte in bytes {
        print!("0x{:02x} ", byte);
    }
    println!("");
}

trait Packet {

}

struct EncryptionRequest {
    server_id : String,
    public_key : Vec<u8>,
    verify_token : Vec<u8>,
}

impl Packet for EncryptionRequest {

}

fn read_u8(stream : &mut std::io::Read) -> std::io::Result<u8> {
    let mut buffer : [u8; 1] = [0];
    try!(stream.read_exact(&mut buffer));
    return Ok(buffer[0]);
}

fn read_varint_u64(stream : &mut std::io::Read) -> std::io::Result<u64> {
    let mut r : u64 = 0;
    loop {
        let b = try!(read_u8(stream));
        r += (b & 0x7F) as u64;
        if (b & 0x80) == 0 { break; }
        r <<= 7;
    }
    return Ok(r);
}

fn read_packet(stream : &mut std::io::Read) -> std::io::Result<Vec<u8>> {
    let length = try!(read_varint_u64(stream));
    let mut buffer = vec![0; length as usize];
    try!(stream.read_exact(&mut buffer));
    return Ok(buffer)
}

fn main() {
    let settings = read_settings();
    let server_address : String = settings["server"].as_str().unwrap().to_owned();
    let server_port : u16 = settings["server_port"].as_i64().unwrap() as u16;
    let server_name : String = server_address.to_owned() + ":" + &server_port.to_string();
    let username : String = settings["username"].as_str().unwrap().to_owned();

    println!("Connecting to {}...", server_name);
    let mut stream = TcpStream::connect(&*server_name).unwrap();
    println!("Connected.");

    let handshake = Handshake {
        protocol_version: 107,  // current protocol version
        server_address: server_address,
        server_port: server_port,
        next_state: 2, // login
    };
    let login_start = LoginStart {
        username: username,
    };
    print_bytes(&handshake.encode());
    stream.write(&handshake.encode()).unwrap();
    print_bytes(&login_start.encode());
    stream.write(&login_start.encode()).unwrap();

    let packet = read_packet(&mut stream).unwrap();
    println!("Packet:");
    print_bytes(&packet);

    let mut buffer : [u8; 100] = [0; 100];
    let byte_count = stream.read(&mut buffer).unwrap();
    println!("Left-over bytes:");
    print_bytes(&buffer[0..byte_count]);
}
