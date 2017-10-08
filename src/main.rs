use std::io::Write;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let mut str = String::new();;
    let mut stream = TcpStream::connect("127.0.0.1:10000").unwrap();
    stream.read_to_string(&mut str).unwrap();
    println!("{}", str);
}
