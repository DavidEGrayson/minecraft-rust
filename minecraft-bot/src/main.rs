extern crate yaml_rust;
use std::net::TcpStream;
use std::io::prelude::*;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

mod protocol;
use protocol::pack;
use protocol::packet;
use protocol::login;
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

    match login::login(&mut stream, &server_address, server_port, &username) {
        Result::Ok(_) => (),
        Result::Err(e) => {
            println!("Failed to log in: {}.", e);
            return;
        }
    };

    println!("Successfully logged in.");
}
