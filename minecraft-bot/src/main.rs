extern crate yaml_rust;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;
use yaml_rust::YamlLoader;

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

fn main() {
    let settings = read_settings();
    let name : &str = settings["server"].as_str().unwrap();
    println!("Connecting to {}...", name);
    let mut stream = TcpStream::connect(name).unwrap();
    let mut buffer : [u8; 10] = [0; 10];
    let mut handshake = [
        0,  // packet ID 0: handshake
        47, // protocol version
        // TODO: finish this
        ];
    stream.read(&mut buffer).unwrap();
}
