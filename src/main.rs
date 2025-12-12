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
        // let _http_request: Vec<_> = reader
        //     .lines()
        //     .map(|result| result.unwrap())
        //     .take_while(|line| !line.is_empty())
        //     .collect();

        if request_route == "GET / HTTP/1.1" {
            let status_line = "HTTP/1.1 200 OK\r\n\r\n";
            let contents = fs::read_to_string("public/response.html").unwrap();
            
            let response = format!("{status_line} {contents}");
            
            stream.write_all(response.as_bytes()).unwrap();
        } else {
            let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            let contents = fs::read_to_string("public/404.html").unwrap();

            let response = format!("{status_line} {contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
    }

}