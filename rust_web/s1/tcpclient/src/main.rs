use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap(); //传输需要用原始字节

    let mut buffer = [0; 5]; // 服务器把发送数据原封不动返回
    stream.read(&mut buffer).unwrap();
    println!(
        "Response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    ); // 把原始字节转换成utf8字符串
}
