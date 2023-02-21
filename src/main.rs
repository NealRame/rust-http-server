use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

const INDEX_FALLBACK: &str = "\
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

const ERROR_404_FALLBACK: &str = "\
<!DOCTYPE html>\
<html lang='en'>\
    <head>\
        <meta charset='utf-8'>\
        <title>404 Not found</title>\
    </head>\
    <body>\
        <h1>404</h1>\
        <p>Page not found</p>\
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
    let request_line = but_reader.lines().next().unwrap().unwrap();
    let (status, file, fallback) =
        if request_line == "GET / HTTP/1.1" {
            (
                "HTTP/1.1 200 OK",
                "index.html",
                INDEX_FALLBACK,
            )
        } else {
            (
                "HTTP/1.1 400 NOT FOUND",
                "404.html",
                ERROR_404_FALLBACK,
            )
        };
    let content = fs::read_to_string(file).unwrap_or(String::from(fallback));
    let header_content_length = format!("Content-Length: {}", content.len());
    let res = format!("{status}\r\n{header_content_length}\r\n\r\n{content}");
    stream.write_all(res.as_bytes()).unwrap();
}
