pub enum Packet {
    Handshake(Handshake),
    LoginStart(LoginStart),
    EncryptionRequest(EncryptionRequest),
    Unknown,
}

pub struct Handshake {
    pub protocol_version : u32,
    pub server_address : String,
    pub server_port: u16,
    pub next_state : u32,  // 1 for status, 2 for login
}

pub struct LoginStart {
    pub username : String
}

pub struct EncryptionRequest {
    pub server_id : String,
    pub public_key : Vec<u8>,
    pub verify_token : Vec<u8>,
}
