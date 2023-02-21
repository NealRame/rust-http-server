use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

const FALLBACK_CONTENT: &str = "\
<!DOCTYPE html>\
<html lang='en'>\
    <head>\
        <meta charset='utf-8'>\
        <title>Hello!</title>\
    </head>\
    <body>\
        <h1>Hello!</h1>\
        <p>Hi from Rust</p>\
    </body>\
</html>";

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
    let req: Vec<_> = but_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("hello.html").unwrap_or(String::from(FALLBACK_CONTENT));
    let header_content_length = format!("Content-Length: {}", content.len());

    let res = format!("{status}\r\n{header_content_length}\r\n\r\n{content}");

    // println!("Request: {:#?}", http_request);

    stream.write_all(res.as_bytes()).unwrap();
}
