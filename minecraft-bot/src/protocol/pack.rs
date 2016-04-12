use std::io;
use std::str;

pub fn wrap_packet(x : Vec<u8>) -> Vec<u8>
{
    let mut r = encode_varint(x.len() as u32);
    r.extend(x);
    r
}

pub fn encode_varint(n : u32) -> Vec<u8> {
    if n > 127 {
        let mut r : Vec<u8> = vec![(n as u8) | 0x80];
        r.extend(&encode_varint(n >> 7));
        r
    }
    else {
        return vec![n as u8];
    }
}

pub fn encode_u16(n : u16) -> Vec<u8> {
    return vec![(n >> 8) as u8, n as u8];
}

pub fn encode_string(s : &str) -> Vec<u8>
{
    let mut r : Vec<u8> = encode_varint(s.len() as u32); // TODO: don't use 'as' here
    r.extend(s.to_owned().into_bytes());
    r
}

pub fn read_u8(stream : &mut io::Read) -> io::Result<u8> {
    let mut buffer : [u8; 1] = [0];
    try!(stream.read_exact(&mut buffer));
    return Ok(buffer[0]);
}

pub fn read_varint_u64(stream : &mut io::Read) -> io::Result<u64> {
    let mut r : u64 = 0;
    loop {
        let b = try!(read_u8(stream));
        r += (b & 0x7F) as u64;
        if (b & 0x80) == 0 { break; }
        r <<= 7;
    }
    return Ok(r);
}

pub fn read_var_bytes(stream : &mut io::Read) -> io::Result<Vec<u8>> {
    let length = try!(read_varint_u64(stream));
    let mut buffer = vec![0; length as usize];
    try!(stream.read_exact(&mut buffer));
    return Ok(buffer)
}

pub fn read_packet(stream : &mut io::Read) -> io::Result<Vec<u8>> {
    return read_var_bytes(stream);
}

pub fn read_string(stream : &mut io::Read) -> io::Result<String> {
    let mut buffer = try!(read_var_bytes(stream));
    match str::from_utf8(&buffer) {
        Ok(v) => return Ok(v.to_owned()),
        Err(e) => panic!("Invalid UTF-8"),  // TODO: proper error handling
    };
}