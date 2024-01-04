mod config;
mod window;
use iced::{Application, Settings};
use window::BatteryStatus;

fn main() {
    BatteryStatus::run(Settings {
        window: iced::window::Settings {
            size: (550, 300),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    })
    .expect("Iced-Battery");
}
