use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    start_datagrid();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..]);
    let req: Vec<&str> = message.split(' ').collect();
    let my_date = operate(String::from(req[1]), String::from(req[2]), req[0]);
    println!("Request: {:?}", my_date);
}

fn write() {
    let mut stream =
        TcpStream::connect("127.0.0.1:7878").expect("Couldn't connect to the server...");
    let request = String::from("SET a hoge ");
    stream.write(request.as_bytes()).unwrap();
}

fn start_datagrid() {
    let lister = TcpListener::bind("127.0.0.1:7878").unwrap();
    write();
    for stream in lister.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn operate(key: String, value: String, operation: &str) -> HashMap<String, String> {
    let mut data: HashMap<String, String> = HashMap::new();
    if operation == "SET" {
        data.insert(key, value);
    }

    return data;
}
