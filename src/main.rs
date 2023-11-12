use std::net::TcpListener;
use std::io::{Read,Write};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1234").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        println!("opened stream");

        let mut buf = String::new();
        //stream.read_to_string(&mut buf).unwrap();
        println!("{}", buf);

        stream.write("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nhi\r\n".as_bytes()).unwrap();
    }
}
