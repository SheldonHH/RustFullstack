use std::net::TcpListener;
use std::io::{Read, Write};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3001").unwrap();
    // unwrap简单的说就是如果有错误就panic
    println!("Running on port 3000 ...");

    // let result= listener.accept().unwrap();
    // incoming()是一个迭代器，返回一个流，会监听连接
    for stream in listener.incoming() {
        // let _stream = stream.unwrap();
        let mut stream = stream.unwrap(); 
        println!("Connection established!");

        let mut buffer = [0; 1024]; // 1kb

        stream.read(&mut buffer).unwrap();
        stream.write(&buffer).unwrap();
    }
}

