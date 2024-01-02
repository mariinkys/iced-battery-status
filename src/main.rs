mod window;
use iced::{Application, Settings};
use window::State;

fn main() {
    State::run(Settings::default()).expect("Iced-Battery");
}
