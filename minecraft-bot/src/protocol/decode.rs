use super::pack;
use super::packet;
use super::packet::Packet;
use std::io;

pub fn decode(mut raw : Vec<u8>) -> Packet {
    let mut slice : &[u8] = raw.as_mut_slice();
    let reader : &mut io::Read = &mut slice;
    let packet_id = pack::read_varint_u64(reader).unwrap(); // TODO: proper error handling

    // TODO: need to take a state parameter (Login, Status, or Play)
    // because the packet IDs are different in different states

    println!("packet id = {}", packet_id);
    if packet_id == 0x00 {
        Packet::Disconnect(decode_disconnect(reader))
    }
    else if packet_id == 0x01 {
        Packet::EncryptionRequest(decode_encryption_request(reader))
    }
    else {
        Packet::Unknown
    }
}

fn decode_disconnect(stream : &mut io::Read) -> packet::Disconnect {
    // TODO: proper error handling
    let reason = pack::read_string(stream).unwrap();
    packet::Disconnect { reason: reason }
}

fn decode_encryption_request(stream : &mut io::Read) -> packet::EncryptionRequest {
    // TODO: proper error handling
    let server_id : String = pack::read_string(stream).unwrap();
    let public_key : Vec<u8> = pack::read_var_bytes(stream).unwrap();
    let verify_token : Vec<u8> = pack::read_var_bytes(stream).unwrap();
    packet::EncryptionRequest {
        server_id: server_id,
        public_key: public_key,
        verify_token: verify_token,
    }
}

