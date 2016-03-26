extern crate yaml_rust;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;
use yaml_rust::YamlLoader;

fn encode_varint(n : u32) -> Vec<u8> {
    if n > 127 {
        let mut r : Vec<u8> = vec![(n as u8) | 0x80];
        let rest : Vec<u8> = encode_varint(n >> 7);
        r.extend(&rest);
        r
    }
    else {
        return vec![n as u8];
    }
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
        r.extend(&encode_varint(self.server_port as u32));
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

fn main() {
    let settings = read_settings();
    let server_address : &str = settings["server"].as_str().unwrap();
    let server_port : u16 = settings["server_port"].as_i64().unwrap() as u16;
    let name = server_address.to_string() + ":" + &server_port.to_string();
    println!("Connecting to {}...", name);
    let mut stream = TcpStream::connect(&*name).unwrap();
    println!("Connected.");
    let mut buffer : [u8; 10] = [0; 10];
    let handshake = Handshake {
        protocol_version: 107,  // current protocol version
        server_address: server_address.to_owned(),
        server_port: server_port,
        next_state: 2, // login
    };
    let login_start = LoginStart {
        username: "Elavid".to_owned(),
    };
    print_bytes(&handshake.encode());
    stream.write(&handshake.encode()).unwrap();
    print_bytes(&login_start.encode());
    stream.write(&login_start.encode()).unwrap();
    let byte_count = stream.read(&mut buffer).unwrap();
    print_bytes(&buffer[0..byte_count]);
    std::thread::sleep(std::time::Duration::new(5, 0));
}
