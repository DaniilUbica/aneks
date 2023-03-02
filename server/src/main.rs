use std::net::TcpListener;
use std::thread::spawn;

pub mod send_mail;
pub mod database;

use send_mail::send_mail::*;
use database::database::*;

use anekdot_sru::*;

fn main() {
    create_table(database::TABLE_NAME, ["email"].to_vec());

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Listening started, ready to accept");
    loop {
        let (stream, addr) = listener.accept().unwrap();

        println!("New connection: {}", addr);

        spawn(move | |{
            loop {
                let q = handle_client(stream.try_clone().unwrap());
                if !q {
                    return;
                }
            }
        });
    }

}
