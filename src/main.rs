use std::{
    io::{BufReader, prelude::*}, 
    net::{TcpListener, TcpStream}
};

fn main() {
    let server = TcpListener::bind("127.0.0.1:3000").unwrap();

    for server_stream in server.incoming() {
        let server_stream = server_stream.unwrap();

        handle_request(server_stream);
    }

    fn handle_request(stream: TcpStream) {
        let reader = BufReader::new(&stream);
        let http_request: Vec<_> = reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        
        println!("Request: {http_request:#?}");
    }

}