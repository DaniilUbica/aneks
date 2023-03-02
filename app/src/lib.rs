use rand::Rng;
use std::{net::TcpStream, io::Write};

pub fn generate_code(length: i32) -> String {
    let chars = "QWERTYUIOPASDFGHJKLZXCVBNM";
    let mut code = String::new();
    for _i in 0..length {
        let rndm_index = rand::thread_rng().gen_range(0..chars.len());
        code.push(chars.chars().nth(rndm_index).unwrap());
    }

    code
}

pub fn write_to_server(ip: &str, text: &str) {
    let mut stream = TcpStream::connect(ip).expect("Cann't connect to server");

    stream.write(text.as_bytes()).expect("Cann't write message to server");
}