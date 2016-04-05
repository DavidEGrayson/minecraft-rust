use protocol::pack;
use protocol::packet;

pub trait EncodablePacket {
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
