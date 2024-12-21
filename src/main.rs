use std::net::TcpListener;

const BIND: &str = "127.0.0.1:7878";

fn main() {
    let listener = TcpListener::bind(BIND).unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
    }
}
