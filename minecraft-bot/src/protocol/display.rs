use super::packet;
use std::fmt;

fn display_unknown_case(f : &mut fmt::Formatter) -> fmt::Result {
    try!(f.write_str("displaying this type of packet is not implemented"));
    Ok(())
}

impl fmt::Display for packet::Packet
{
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            packet::Packet::EncryptionRequest(e) => Ok(()), //e.fmt(f),
            _ => display_unknown_case(f),
        }
    }
}

impl fmt::Display for packet::EncryptionRequest
{
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        try!(f.write_str("EncryptionRequest"));
        Ok(())
    }
}