/// all comments on this file are used as notes for the webserver that I will create

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listen = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listen.incoming() {

        // more could be used in place of unwrap, handle errors instead of just panicking
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // there's probably a better way to handle this error
    let request = buf_reader.lines().next().unwrap().unwrap();


    let (status, fname) = match &request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./src/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 OK", "./src/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "./src/404.html"),
    };
    
    let content = fs::read_to_string(fname).unwrap();
    let len = content.len();
    let response = format!("{status}\r\nContent-Length: {len}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}