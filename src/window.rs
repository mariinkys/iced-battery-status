use iced::{
    executor,
    widget::{column, container, text},
    Application, Command, Length, Theme,
};

pub struct State {
    theme: Theme,
    battery_location: String,
}

#[derive(Debug, Clone)]
pub enum Messages {
    UpdateBatteryLocation(String),
}

impl Application for State {
    type Message = Messages;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            State {
                theme: system_theme_mode(),
                battery_location: String::from("/org/freedesktop/UPower/devices/battery_BAT0"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Battery Status")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Messages::UpdateBatteryLocation(data) => self.battery_location = data,
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let title_text = text("Battery Status").size(50);

        container(
            column!(title_text)
                .align_items(iced::Alignment::Center)
                .padding(10),
        )
        .width(Length::Fill)
        .center_x()
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn system_theme_mode() -> Theme {
    match dark_light::detect() {
        dark_light::Mode::Light | dark_light::Mode::Default => Theme::Light,
        dark_light::Mode::Dark => Theme::Dark,
    }
}
