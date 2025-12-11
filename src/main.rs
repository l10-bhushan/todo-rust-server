use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use crate::{
    db::data_base::{TodoDb, TodosStruct},
    helper::http_parse,
    route::return_404_notfound,
};

mod db;
mod helper;
mod route;

const STATUS_LINE: &str = "HTTP/1.1 200 OK";
const BAD_STATUS_LINE: &str = "HTTP/1.1 404 NOT FOUND";

#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    body: String,
}

fn router(stream: &mut TcpStream, db_instance: &mut TodoDb) {
    let http_request = http_parse(&stream);
    match &http_request.method[..] {
        "GET" => match &http_request.path[..] {
            "/health" => {
                let content = route::server_health();
                let response = format!("{STATUS_LINE}\r\n\r\n {content}");
                stream.write_all(response.as_bytes()).unwrap();
            }
            "/todos" => {
                let content = route::get_todos(&db_instance);
                let response = format!("{STATUS_LINE}\r\n\r\n {content:?}");
                stream.write_all(response.as_bytes()).unwrap();
            }
            _ => {
                let response = format!("{BAD_STATUS_LINE}\r\n\r\n");
                stream.write_all(response.as_bytes()).unwrap();
            }
        },
        "POST" => match &http_request.path[..] {
            "/add_todo" => {
                let content = route::add_todo(http_request.body, db_instance);
                let response = format!("{STATUS_LINE}\r\n\r\n {content:?}");
                stream.write_all(response.as_bytes()).unwrap();
            }
            _ => {
                let response = return_404_notfound();
                stream.write_all(response.as_bytes()).unwrap();
            }
        },
        "PUT" => {}
        "DELETE" => match &http_request.path[..] {
            "/delete_todo" => {
                let content = route::delete_todo(http_request.body, db_instance);
                let response = format!("{STATUS_LINE}\r\n\r\n {content:?}");
                stream.write_all(response.as_bytes()).unwrap();
            }
            _ => {
                let response = return_404_notfound();
                stream.write_all(response.as_bytes()).unwrap();
            }
        },
        _ => println!("OTHER requests"),
    }
}

fn main() {
    // Initializing the TCP listener to listen on port 8000
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let mut tododb = TodoDb::new();
    for stream in listener.incoming() {
        println!("Connection established!");
        let mut stream = stream.unwrap();
        router(&mut stream, &mut tododb);
    }
}
