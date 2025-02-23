mod window;

use iced::{
    Size, Task,
    window::{Position, Settings},
};
use window::BatteryStatus;

fn main() -> Result<(), iced::Error> {
    iced::application("Battery Status", BatteryStatus::update, BatteryStatus::view)
        .window(Settings {
            size: Size {
                width: 300.,
                height: 130.,
            },
            position: Position::Centered,
            resizable: true,
            ..Default::default()
        })
        .subscription(BatteryStatus::subscription)
        .theme(BatteryStatus::theme)
        .run_with(|| (BatteryStatus::new(), Task::none()))
}
