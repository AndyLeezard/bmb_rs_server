use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use bmb_rs_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    // ex: ...incoming().take(2) will allow 2 connections and stop listening incoming streams.
    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
        // println!("Connection established!");
    }
    println!("Shutting down.")
}

/*
 * stream parameter is mutable because the 'read' method takes a mutable reference.
 * When you're reading from a stream, some internal states get modified.
 */
fn handle_connection(mut stream: TcpStream) {
    // 1024-bytes-long buffer: large enough to store the basic request for test purposes.
    // for production, this should be able to handle different sizes of requests.
    let mut buffer = [0; 1024];

    // populate the buffer with data from the stream
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Lenth: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    /* println!(
        "Request: {}",
        String::from_utf8_lossy(&buffer[..])
    ) */

    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    //
    // ex: HTTP/1.1 200 OK\r\n\r\n
}
