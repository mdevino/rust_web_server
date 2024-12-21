use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const BIND: &str = "127.0.0.1:7878";

fn main() {
    let listener = TcpListener::bind(BIND).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
