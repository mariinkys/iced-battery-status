mod window;
use iced::{Application, Settings};
use window::State;

fn main() {
    State::run(Settings {
        window: iced::window::Settings {
            size: (500, 300),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    })
    .expect("Iced-Battery");
}
