use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    if let Ok(pool) = ThreadPool::new(4) {
        for stream in listener.incoming().take(2) {
            let stream = stream.unwrap();

            pool.execute(|| {
                handle_connection(stream);
            });
        }
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // reads from connection
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    // sends data downstream connection
    stream.write(response.as_bytes()).unwrap();

    // prevent program from continuing until all bytes are written
    // TcpStream carries internal buffer to minimize OS calls
    stream.flush().unwrap();
}
