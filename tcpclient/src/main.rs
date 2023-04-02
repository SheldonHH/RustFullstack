use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;
fn main() {
    // let _stream = TcpStream::connect("localhost:3000").unwrap();
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap(); // 向服务器发送数据
    let mut buffer = [0; 1024]; // 1kb 


    stream.read(&mut buffer).unwrap(); // 从服务器读取数据
    println!("Response: {}", str::from_utf8(&buffer).unwrap());



    println!("Hello, world!");
}
