mod config;
mod window;
use iced::{Application, Settings, Size};
use window::BatteryStatus;

fn main() {
    BatteryStatus::run(Settings {
        window: iced::window::Settings {
            size: (Size {
                width: 550.0,
                height: 300.0,
            }),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    })
    .expect("Iced-Battery");
}
