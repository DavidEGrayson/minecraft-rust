#[derive(Debug)]
pub enum Packet {
    Handshake(Handshake),
    LoginStart(LoginStart),
    Disconnect(Disconnect),
    EncryptionRequest(EncryptionRequest),
    Unknown,
}

#[derive(Debug)]
pub struct Handshake {
    pub protocol_version : u32,
    pub server_address : String,
    pub server_port: u16,
    pub next_state : u32,  // 1 for status, 2 for login
}

#[derive(Debug)]
pub struct LoginStart {
    pub username : String
}

#[derive(Debug)]
pub struct Disconnect {
    pub reason : String,
}

#[derive(Debug)]
pub struct EncryptionRequest {
    pub server_id : String,
    pub public_key : Vec<u8>,
    pub verify_token : Vec<u8>,
}
