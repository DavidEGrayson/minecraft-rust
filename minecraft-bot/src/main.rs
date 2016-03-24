extern crate yaml_rust;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;
use yaml_rust::YamlLoader;

trait EncodablePacket {
    fn encode(&self) -> Vec<u8>;
}

struct Handshake {
    protocol_version : u32,
    server_address : String,
    server_port: u16,
    next_state : u32,  // 1 for status, 2 for login
}

impl EncodablePacket for Handshake {
    fn encode(&self) -> Vec<u8> {
        vec![1, 2, 3]  // TODO: something real here
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
        println!("byte = {}", byte);
    }
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
    let mut handshake = Handshake {
        protocol_version: 47,  // current protocol version
        server_address: server_address.to_owned(),
        server_port: server_port,
        next_state: 2, // login
    };
    stream.write(&handshake.encode()).unwrap();
    let byte_count = stream.read(&mut buffer).unwrap();
    print_bytes(&buffer[0..byte_count]);
}
