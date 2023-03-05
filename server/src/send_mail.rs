pub mod send_mail {
    use rand::Rng;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};

    pub fn generate_code(length: i32) -> String {
        let chars = "QWERTYUIOPASDFGHJKLZXCVBNM";
        let mut code = String::new();
        for i in 0..length {
            let rndm_index = rand::thread_rng().gen_range(0..chars.len());
            code.push(chars.chars().nth(rndm_index).unwrap());
        }

        code
    }

    pub fn send_mail(text: &str, to: &str) {
        let email = Message::builder()
        .from("Aneks <sender@gmail.com>".parse().unwrap())
        .to(format!("<{to}>").parse().unwrap())
        .body(String::from(text))
        .unwrap();

        let creds = Credentials::new("sender@gmail.com".to_string(), "pass".to_string());

        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}