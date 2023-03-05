use fltk::*;
use fltk::app::frame_color;
use fltk::button::Button;
use fltk::frame::Frame;
use fltk::window::Window;
use fltk::prelude::*;
use regex::Regex;

use aneks_app::*;

const IP: &str = "127.0.0.1:8000";

fn main() {
    let regex: Regex = Regex::new(r"([a-z0-9.-_]+@+[a-z0-9.-_]+.[a-z]+)").unwrap();

    let app = app::App::default();
    let mut wind = Window::new(350, 150, 450, 300, "Анекдоты");

    let conf_code = generate_code(5);

    let mut email_input = input::Input::new(150, 50, 150, 25, "Ваша почта: ");
    email_input.set_frame(enums::FrameType::PlasticDownBox);

    let mut code_input = input::Input::new(150, 150, 150, 25, "Код активации: ");
    code_input.set_frame(enums::FrameType::PlasticDownBox);

    code_input.deactivate();

    let mut confirm_button = Button::new(150, 100, 150, 25, "Подтвердить почту");
    confirm_button.set_frame(enums::FrameType::PlasticDownBox);

    let mut code_button = Button::new(150, 200, 150, 25, "Подтвердить код");
    code_button.set_frame(enums::FrameType::PlasticDownBox);

    let info = Frame::new(75, 250, 300, 25, "");

    wind.end();
    wind.show();

    confirm_button.set_callback({
        let mut code_input = code_input.clone();

        let conf_code = conf_code.clone();
        let mut email_input = email_input.clone();
        let mut info = info.clone();

        move |_| {
            code_input.activate();
            
            code_input.activate();
            if regex.is_match(&email_input.value()) {
                write_to_server(IP, &format!("C {} {}", conf_code, email_input.value()));

                info.set_label("Код подтверждения был отправлен \nна Вашу почту");
            }
            else {
                email_input.set_value("");
                info.set_label("Пожалуйста, введите корректный \nemail");
            }
    }});

    code_button.set_callback({
        let mut info = info.clone();

        move |_| {
        if code_input.value() == conf_code {
            write_to_server(IP, &format!("R {}", email_input.value()));
            info.set_label("Ура, теперь вы будете получать \nанекдоты каждый день в 9 утра по МСК");
        }
        else {
            code_input.set_value("");
            info.set_label("Неверный код");
        }
        
    }});

    app.run().unwrap();
}
