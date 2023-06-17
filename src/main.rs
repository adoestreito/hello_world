use std::io::{self, Read, Write};
use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::env;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = include_str!("../static/index.html");

    let response = format!("{}{}", response, contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr)?;

    println!("Server listening on http://0.0.0.0:{}", port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(move || {
            handle_client(stream);
        });
    }

    Ok(())
}

