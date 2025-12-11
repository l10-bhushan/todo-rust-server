use crate::HttpRequest;
use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

pub fn http_parse(stream: &TcpStream) -> HttpRequest {
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
                body_vec_size = vec_size;
            }
            current_line
        })
        .take_while(|line| !line.is_empty())
        .collect();
    let mut buffer = vec![0; body_vec_size]; //New Vector with size of Content   
    request.read_exact(&mut buffer).unwrap(); //Get the Body Content.
    let body = String::from_utf8(buffer).unwrap();
    let method_n_post: Vec<&str> = lines[0].split(" ").map(|item| item).collect();
    HttpRequest {
        method: String::from(method_n_post[0]),
        path: String::from(method_n_post[1]),
        body,
    }
}

pub fn body_parser(body: String) -> String {
    let new: Vec<String> = body
        .trim()
        .lines()
        .collect::<String>()
        .replace("{", "")
        .replace("}", "")
        .replace("\"", "")
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();
    let task: Vec<_> = new[0].split(":").collect();
    task[1].trim().to_string()
}
