use iced::{
    alignment::{Horizontal, Vertical},
    executor,
    widget::{container, text, Button, Column, Container, Row, Space},
    Application, Command, Length, Padding, Theme,
};
use starship_battery::Battery;

use crate::theme::{ButtonStyle, ContainerStyle};

pub struct BatteryStatus {
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Messages {
    ToggleTheme,
}

impl Application for BatteryStatus {
    type Message = Messages;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self { theme: Theme::Dark }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Battery Status")
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Messages::ToggleTheme => {
                self.theme = if self.theme == Theme::Light {
                    Theme::Dark
                } else {
                    Theme::Light
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let content = content_page();

        let wrapper = Column::new()
            .width(Length::Fill)
            .push(page_header())
            .push(content);

        container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from(10))
            .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
            .into()
    }
}

// Page Header
fn page_header() -> Container<'static, Messages> {
    let toggle_button = toggle_theme_button("T", Messages::ToggleTheme);

    let header = Row::new()
        .push(Space::new(Length::Fill, 0))
        .push(toggle_button)
        .width(Length::Fill)
        .align_items(iced::Alignment::End)
        .spacing(10);

    container(header).align_x(Horizontal::Right)
}

// Content Page
fn content_page() -> Container<'static, Messages> {
    let column = format_battery_info();

    container(column)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
}

// Toggle Theme Button
fn toggle_theme_button(name: &str, event: Messages) -> Button<Messages> {
    Button::new(
        text(name)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .size(15),
    )
    .on_press(event)
    .width(Length::Fixed(30.0))
    .height(Length::Fixed(30.0))
    .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Standard)))
}

fn format_battery_info() -> Column<'static, Messages> {
    match get_battery_info() {
        Ok(battery) => {
            //Format the battery data
            let battery_name: &str = battery.model().unwrap_or("Battery name not available");
            let current_battery: f32 = battery
                .state_of_charge()
                .get::<starship_battery::units::ratio::percent>();
            let max_capacity_designed: f32 = battery
                .energy_full_design()
                .get::<starship_battery::units::energy::watt_hour>();
            let max_capacity: f32 = battery
                .energy_full()
                .get::<starship_battery::units::energy::watt_hour>();
            let battery_health: f32 = battery
                .state_of_health()
                .get::<starship_battery::units::ratio::percent>();

            //Format Each Row
            let current_battery_row = Row::new()
                .spacing(3.0)
                .push(
                    text("Current Battery:")
                        .size(15)
                        .vertical_alignment(Vertical::Center),
                )
                .push(
                    text(format!("{:.2}%", current_battery))
                        .size(15)
                        .vertical_alignment(Vertical::Center),
                )
                .align_items(iced::Alignment::Center);

            let max_capacity_designed_row = Row::new()
                .spacing(3.0)
                .push(
                    text("Designed Max Capacity")
                        .size(15)
                        .vertical_alignment(Vertical::Center),
                )
                .push(
                    text(format!("{} Wh", max_capacity_designed))
                        .size(15)
                        .vertical_alignment(Vertical::Center),
                )
                .align_items(iced::Alignment::Center);

            let max_capacity_row = Row::new()
                .spacing(3.0)
                .push(
                    text("Actual Max Capacity:")
                        .size(15)
                        .vertical_alignment(Vertical::Center),
                )
                .push(
                    text(format!("{} Wh", max_capacity))
                        .size(15)
                        .vertical_alignment(Vertical::Center),
                )
                .align_items(iced::Alignment::Center);

            let battery_health_row = Row::new()
                .spacing(3.0)
                .push(
                    text("Battery Health:")
                        .size(15)
                        .vertical_alignment(Vertical::Center),
                )
                .push(
                    text(format!("{:.2}%", battery_health))
                        .size(15)
                        .vertical_alignment(Vertical::Center),
                )
                .align_items(iced::Alignment::Center);

            Column::new()
                .push(text(battery_name).size(24))
                .push(current_battery_row)
                .push(max_capacity_designed_row)
                .push(max_capacity_row)
                .push(battery_health_row)
                .align_items(iced::Alignment::Center)
        }
        Err(_err) => Column::new().push(text("Error Getting Battery").size(24)),
    }
}

fn get_battery_info() -> Result<Battery, starship_battery::Error> {
    let manager = starship_battery::Manager::new()?;

    let mut batteries = Vec::new();

    for maybe_battery in manager.batteries()? {
        match maybe_battery {
            Ok(bat) => batteries.push(bat),
            Err(err) => return Err(err),
        }
    }

    // Return the first battery found
    batteries.into_iter().next().ok_or_else(|| {
        starship_battery::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Battery Not Found",
        ))
    })
}
