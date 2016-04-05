pub fn print_bytes(bytes : &[u8]) {
    for byte in bytes {
        print!("0x{:02x} ", byte);
    }
    println!("");
}
