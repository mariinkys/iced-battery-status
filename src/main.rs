// SPDX-License-Identifier: GPL-3.0-only

mod i18n;
mod window;

use iced::{
    Size, Task,
    window::{Position, Settings},
};
use window::BatteryStatus;

fn main() -> Result<(), iced::Error> {
    // Get the system's preferred languages.
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    i18n::init(&requested_languages);

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
