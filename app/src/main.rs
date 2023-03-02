use fltk::*;
use fltk::button::Button;
use fltk::window::Window;
use fltk::prelude::*;

use aneks_app::*;

const IP: &str = "127.0.0.1:8000";

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(350, 150, 800, 400, "Aneks");

    let conf_code = generate_code(5);

    let mut email_input = input::Input::new(500, 200, 150, 25, "Your email: ");
    email_input.set_frame(enums::FrameType::PlasticDownBox);

    let mut code_input = input::Input::new(500, 300, 150, 25, "Activation code: ");
    code_input.set_frame(enums::FrameType::PlasticDownBox);

    code_input.deactivate();

    let mut confirm_button = Button::new(500, 250, 150, 25, "Confirm email");
    confirm_button.set_frame(enums::FrameType::PlasticDownBox);

    let mut code_button = Button::new(500, 350, 150, 25, "Confirm code");
    code_button.set_frame(enums::FrameType::PlasticDownBox);

    wind.end();
    wind.show();

    confirm_button.set_callback({
        let mut code_input = code_input.clone();

        let conf_code = conf_code.clone();
        let email_input = email_input.clone();

        move |_| {
        code_input.activate();

        code_input.activate();

        write_to_server(IP, &format!("C {} {}", conf_code, email_input.value()));
    }});

    code_button.set_callback(move |_| {
        if code_input.value() == conf_code {
            write_to_server(IP, &format!("R {}", email_input.value()));
        }
        else {
            code_input.set_value("");
        }
        
    });

    app.run().unwrap();
}
