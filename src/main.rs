use core::panic;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    usize,
};

use rust_web_server::{ThreadPool, BIND_ADDRESS};

fn main() {
    let listener = TcpListener::bind(BIND_ADDRESS).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let (status_line, filename) = match http_request[0].as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent=Length: {length}\r\n\r\n{contents}");

    let response_lines: Vec<_> = response.lines().filter(|line| !line.is_empty()).collect();
    println!("REQUEST:\n{http_request:#?}\nRESPONSE:\n{response_lines:#?}\n\n");
    stream.write_all(response.as_bytes()).unwrap();
}
