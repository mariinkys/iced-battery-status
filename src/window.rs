use std::time::Duration;

use iced::{
    Alignment, Length, Padding, Subscription, Theme, time,
    widget::{Column, Row, container, text},
};
use starship_battery::{Battery, Manager};

pub struct BatteryStatus {
    theme: Theme,
    manager: Manager,
    battery: Option<Battery>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SystemThemeMode(Theme),
    GetBattery,
}

impl BatteryStatus {
    pub fn new() -> Self {
        let theme = system_theme_mode();
        let manager = starship_battery::Manager::new().unwrap();
        let battery: Option<Battery> = manager
            .batteries()
            .expect("Failed to get batteries")
            .filter_map(|maybe_battery| maybe_battery.ok())
            .next();

        Self {
            theme,
            manager,
            battery,
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let content = match &self.battery {
            Some(battery) => {
                //Format the battery data
                let battery_name: &str = battery.model().unwrap_or("Battery name not available");
                let current_battery: f32 = battery
                    .state_of_charge()
                    .get::<starship_battery::units::ratio::percent>();
                let max_capacity_designed: f32 = battery
                    .energy_full_design()
                    .get::<starship_battery::units::energy::watt_hour>(
                );
                let max_capacity: f32 = battery
                    .energy_full()
                    .get::<starship_battery::units::energy::watt_hour>();
                let battery_health: f32 = battery
                    .state_of_health()
                    .get::<starship_battery::units::ratio::percent>();

                //Format Each Row
                let current_battery_row = Row::new()
                    .spacing(3.0)
                    .push(text("Current Battery:").size(15).align_y(Alignment::Center))
                    .push(
                        text(format!("{:.2}%", current_battery))
                            .size(15)
                            .align_y(Alignment::Center),
                    )
                    .align_y(Alignment::Center);

                let max_capacity_designed_row = Row::new()
                    .spacing(3.0)
                    .push(
                        text("Designed Max Capacity")
                            .size(15)
                            .align_y(Alignment::Center),
                    )
                    .push(
                        text(format!("{} Wh", max_capacity_designed))
                            .size(15)
                            .align_y(Alignment::Center),
                    )
                    .align_y(Alignment::Center);

                let max_capacity_row = Row::new()
                    .spacing(3.0)
                    .push(
                        text("Actual Max Capacity:")
                            .size(15)
                            .align_y(Alignment::Center),
                    )
                    .push(
                        text(format!("{} Wh", max_capacity))
                            .size(15)
                            .align_y(Alignment::Center),
                    )
                    .align_y(Alignment::Center);

                let battery_health_row = Row::new()
                    .spacing(3.0)
                    .push(text("Battery Health:").size(15).align_y(Alignment::Center))
                    .push(
                        text(format!("{:.2}%", battery_health))
                            .size(15)
                            .align_y(Alignment::Center),
                    )
                    .align_y(Alignment::Center);

                Column::new()
                    .push(text(battery_name).size(24))
                    .push(current_battery_row)
                    .push(max_capacity_designed_row)
                    .push(max_capacity_row)
                    .push(battery_health_row)
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
            }
            None => Column::new().push(
                text("Error Getting Battery")
                    .size(24)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .width(Length::Fill)
                    .height(Length::Fill),
            ),
        };
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from(10))
            .into()
    }

    // Handle appliaction messages
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::GetBattery => {
                self.battery = self
                    .manager
                    .batteries()
                    .expect("Failed to get batteries")
                    .filter_map(|maybe_battery| maybe_battery.ok())
                    .next();
            }

            Message::SystemThemeMode(theme) => {
                self.theme = theme;
            }
        }
        iced::Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // Update the battery info every 30 seconds
        Subscription::batch([
            time::every(Duration::from_secs(30)).map(|_| Message::GetBattery),
            time::every(Duration::from_secs(30))
                .map(|_| Message::SystemThemeMode(system_theme_mode())),
        ])
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn system_theme_mode() -> Theme {
    match dark_light::detect() {
        Ok(dark_light::Mode::Light) | Ok(dark_light::Mode::Unspecified) => Theme::Light,
        Ok(dark_light::Mode::Dark) => Theme::Dark,
        Err(err) => {
            eprintln!("{err}");
            Theme::Dark
        }
    }
}
