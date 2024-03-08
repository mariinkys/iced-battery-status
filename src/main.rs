mod theme;
mod window;
use iced::{Application, Settings, Size};
use window::BatteryStatus;

fn main() {
    BatteryStatus::run(Settings {
        window: iced::window::Settings {
            size: (Size {
                width: 300.0,
                height: 200.0,
            }),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    })
    .expect("Battery Status");
}
