use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};

fn main() {
    let listener =
        TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Listening on port http://127.0.0.1:7878 ...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);

    }
}

fn handle_connection(mut stream: TcpStream) {
    let but_reader = BufReader::new(&stream);
    let http_request: Vec<_> = but_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}