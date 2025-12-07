#[allow(unused_variables)]
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::db::data_base::TodosStruct;

mod db;
mod route;

const STATUS_LINE: &str = "HTTP/1.1 200 OK";
const BAD_STATUS_LINE: &str = "HTTP/1.1 404 NOT FOUND";

struct HttpRequest {
    method: String,
    path: String,
    headers: String,
    body: String,
}

fn http_parse(stream: &TcpStream) -> HttpRequest {
    let mut request = BufReader::new(stream.try_clone().unwrap());
    let mut body_vec_size: usize = 0;
    let lines: Vec<String> = request
        .by_ref()
        .lines()
        .map(|line| {
            let current_line = line.unwrap();
            if current_line.contains("Content-Length") {
                let s: Vec<_> = current_line.split(":").collect();
                let vec_size: usize = s[1].trim().parse().unwrap();
                println!("Vector size is : {}", vec_size);
                body_vec_size = vec_size;
            }
            current_line
        })
        .take_while(|line| !line.is_empty())
        .collect();
    let mut buffer = vec![0; body_vec_size]; //New Vector with size of Content   
    request.read_exact(&mut buffer).unwrap(); //Get the Body Content.
    let body = String::from_utf8(buffer).unwrap();
    println!("Lines : {:?}, {}", lines, body);
    HttpRequest {
        method: String::from(""),
        path: String::from(""),
        headers: String::from(""),
        body: String::from(""),
    }
}

fn handle_connection(mut stream: TcpStream, db_instance: &mut TodosStruct) {
    http_parse(&stream);
    // let request = BufReader::new(&stream);
    // let body: Vec<_> = request
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // let line = &body[0];
    // println!("Line : {} , {:?}", line, body);
    // match &line[..] {
    //     "GET /health HTTP/1.1" => {
    //         let content = route::server_health();
    //         let response = format!("{STATUS_LINE}\r\n\r\n {content}");
    //         stream.write_all(response.as_bytes()).unwrap();
    //     }
    //     "GET /todos HTTP/1.1" => {
    //         let content = route::get_todos(&db_instance);
    //         let response = format!("{STATUS_LINE}\r\n\r\n {content:?}");
    //         stream.write_all(response.as_bytes()).unwrap();
    //     }
    //     "POST /add_todo HTTP/1.1" => {
    //         let content = route::get_todos(&db_instance);
    //         let response = format!("{STATUS_LINE}\r\n\r\n {content:?}");
    //         stream.write_all(response.as_bytes()).unwrap();
    //     }
    //     _ => {
    //         let response = format!("{BAD_STATUS_LINE}\r\n\r\n");
    //         stream.write_all(response.as_bytes()).unwrap();
    //     }
    // }
    let response = format!("{BAD_STATUS_LINE}\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    println!("Todo - server");
    let mut db_instance = db::data_base::TodosStruct::new();
    db_instance.add_todo(String::from("Complete web server"));
    db_instance.add_todo(String::from("Implement a library"));

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        println!("Connection established!");
        let stream = stream.unwrap();
        handle_connection(stream, &mut db_instance);
    }
}
