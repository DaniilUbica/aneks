use std::io::Read;
use std::net::TcpStream;

pub mod database;
pub mod send_mail;

pub use crate::database::*;
pub use crate::send_mail::*;

pub fn get_anek() -> String {
    let text = reqwest::blocking::get("http://rzhunemogu.ru/RandJSON.aspx?CType=1").unwrap().text().unwrap();

    String::from(&text[12..&text.len()-2])
}

pub fn handle_client(mut stream: TcpStream) -> bool {
    let mut buf = vec![0; 128];
    let n = stream.read(&mut buf).unwrap();
    let message = String::from_utf8(buf.clone()).unwrap();

    println!("New message from {}: {}", stream.peer_addr().unwrap(), message);

    let mut q = String::new();
    for i in message.chars() {
        if i != '\0' {
            q.push(i);
        }
    }
    
    if q.len() > 0 {
        if q.chars().nth(0).unwrap() == 'C' {
            let data: Vec<&str> = q.split_whitespace().collect();

            send_mail::send_mail::send_mail(&format!("Ваш код подтверждения: \n\n{}", data[1]), data[2]);
        }
        if q.chars().nth(0).unwrap() == 'R' {
            let data: Vec<&str> = q.split_whitespace().collect();

            database::database::add_record([data[1]].to_vec());
        }
    }
    if n == 0 {
        return false;
    }

    return true;
}