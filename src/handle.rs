
use std::net::{TcpStream};
use std::io::{prelude::*, BufReader};
use std::fs;

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Received request: {:?}", http_req);

    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("index.html").unwrap_or_else(|_| {
        "Error: Could not read the file.".to_string()
    });
    let length = content.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{content}"
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send response: {}", e);
    }
}