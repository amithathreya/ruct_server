
use std::net::{TcpStream};
use std::io::{prelude::*, BufReader};
use std::fs;

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    if request_line == "GET / HTTP/1.1" {
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
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let content = fs::read_to_string("404.html").unwrap_or_else(|_| {
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
}