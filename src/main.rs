use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let host = "0.0.0.0";
    let port = 32101;

    let tcp_uri = format!("{}:{}", host, port);

    println!("Start Server: {}", tcp_uri);

    let listener = TcpListener::bind(tcp_uri).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    stream.write(&buffer).unwrap();
    stream.flush().unwrap();
}
