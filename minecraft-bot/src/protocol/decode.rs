use super::pack;
use super::packet;
use super::packet::Packet;

pub fn decode(raw : Vec<u8>) -> Packet {
    let login_start = packet::LoginStart {
        username: "ABC".to_owned(),
    };
    return Packet::LoginStart(login_start);
}