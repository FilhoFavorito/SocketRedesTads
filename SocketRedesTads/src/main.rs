// Palavras reservadas
// <ACK> - nome aceito
// <NACK> -nome não aceito(já em uso ou restrito)
// <ALL> - mensagem para todos
// <SAIR> - finaliza conexão cliente-servidor
use regex::Regex;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:42000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if let Some(name) = extract_name(&request_line) {
        let mut template = fs::read_to_string("hello.html").unwrap();
        template = template.replace("{{name}}", &name);

        let length = template.len();
        let status_line = "HTTP/1.1 200 OK";

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{template}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn extract_name(request_line: &str) -> Option<String>  {
    let re = Regex::new(r"^GET /([^/\s]+) HTTP/1\.1$").unwrap();
    if let Some(caps) = re.captures(request_line) {
        return Some(caps[1].to_string());
    }
    None
}