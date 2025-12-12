use std::{
    fs,
    io::{BufReader, prelude::*}, 
    net::{TcpListener, TcpStream}
};

fn main() {
    let server = TcpListener::bind("127.0.0.1:3000").unwrap();

    for server_stream in server.incoming() {
        let server_stream = server_stream.unwrap();

        handle_request(server_stream);
    }

    fn handle_request(mut stream: TcpStream) {
        let reader = BufReader::new(&stream);

        let request_route = reader.lines().next().unwrap().unwrap();

        let (status_line, file) = if request_route == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "public/response.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "public/404.html")
        };

        let contents = fs::read_to_string(file).unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }

}