use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

struct DataGrid {
    data: HashMap<String, String>,
}

impl DataGrid {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.data.get(&key)
    }
}

fn handle_connection(mut stream: TcpStream, data_grid: &mut DataGrid) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..]);
    let req: Vec<&str> = message.split(' ').collect();
    operate(
        String::from(req[1]),
        String::from(req[2]),
        req[0],
        data_grid,
    );
    println!("Request: {:?}", data_grid.get("a".to_string()));
}

fn write() {
    let mut stream =
        TcpStream::connect("127.0.0.1:7878").expect("Couldn't connect to the server...");
    let request = String::from("SET a hoge ");
    stream.write(request.as_bytes()).unwrap();
}

fn start_datagrid(data_grid: &mut DataGrid) {
    let lister = TcpListener::bind("127.0.0.1:7878").unwrap();
    write();
    for stream in lister.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, data_grid);
    }
}

fn operate(key: String, value: String, operation: &str, data_grid: &mut DataGrid) {
    if operation == "SET" {
        data_grid.set(key, value);
    }
}

fn main() {
    let mut data_grid = DataGrid::new();
    start_datagrid(&mut data_grid);
}
