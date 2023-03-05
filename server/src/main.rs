use std::net::TcpListener;
use std::thread::spawn;
use chrono::{Datelike, Timelike, Utc};

pub mod send_mail;
pub mod database;

use database::database::*;

use anekdot_sru::*;

fn main() {
    create_table(database::database::get_table_name(), ["email"].to_vec());

    let now = Utc::now();
    let (is_pm, hour) = now.hour12();

    let users = get_emails();

    println!("{:?}", users);

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Listening started, ready to accept");

    let mut flag = true;
    loop {
        if flag {
            for user in &users {
                if hour + 3 == 9 && !is_pm {
                    send_mail::send_mail::send_mail(&format!("Дарим Вам заряд хорошего настроения!!! \n\nВаш анекдот на сегодня:\n\n{}", get_anek()), user);
                }
            }
            flag = false;

            if hour + 3 == 10 {
                flag = true;
            }
        }

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
