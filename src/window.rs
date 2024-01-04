use iced::{
    executor,
    widget::{button, column, container, image, row, text, TextInput},
    Application, Color, Command, Element, Length, Theme,
};

use crate::config::{load_create_config, update_battery_config, BatteryStatusConfiguration};

const FALLBACK_BATTERY_LOCATION: &str = "/org/freedesktop/UPower/devices/battery_BAT0";

#[derive(Debug)]
pub enum BatteryStatus {
    Settings {
        application_settings: BatteryStatusConfiguration,
    },
    Main {
        battery_info: BatteryInfo,
    },
}

impl BatteryStatus {
    pub fn get_application_settings(&mut self) -> Option<&mut BatteryStatusConfiguration> {
        match self {
            BatteryStatus::Settings {
                application_settings,
            } => Some(application_settings),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Messages {
    OpenSettings,
    OpenMain,
    InputNewBatteryLocation(String),
    UpdateBatteryConfig,
}

impl Application for BatteryStatus {
    type Message = Messages;
    type Executor = executor::Default;
    type Flags = BatteryStatusConfiguration;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let application_settings = load_create_config();
        (
            BatteryStatus::Main {
                battery_info: {
                    BatteryInfo::update_battery_info(application_settings.battery_location)
                },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self {
            BatteryStatus::Settings { .. } => "Settings",
            BatteryStatus::Main { .. } => "Main",
        };

        format!("{subtitle} - Battery Status")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Messages::OpenSettings => {
                let loaded_application_settings = load_create_config();
                *self = BatteryStatus::Settings {
                    application_settings: loaded_application_settings,
                };
            }
            Messages::OpenMain => {
                if let Some(application_settings) = self.get_application_settings() {
                    if !application_settings.battery_location.is_empty() {
                        *self = BatteryStatus::Main {
                            battery_info: BatteryInfo::update_battery_info(
                                application_settings.battery_location.to_string(),
                            ),
                        }
                    } else {
                        //Fallback
                        *self = BatteryStatus::Main {
                            battery_info: BatteryInfo::update_battery_info(String::from(
                                FALLBACK_BATTERY_LOCATION,
                            )),
                        }
                    }
                } else {
                    //Fallback
                    *self = BatteryStatus::Main {
                        battery_info: BatteryInfo::update_battery_info(String::from(
                            FALLBACK_BATTERY_LOCATION,
                        )),
                    }
                }
            }
            Messages::InputNewBatteryLocation(data) => {
                if let Some(application_settings) = self.get_application_settings() {
                    application_settings.battery_location = data;
                }
            }
            Messages::UpdateBatteryConfig => {
                if let Some(application_settings) = self.get_application_settings() {
                    let new_json_content = format!(
                        "{{\"battery_location\": \"{}\"}}",
                        application_settings.battery_location
                    );

                    update_battery_config(new_json_content);
                    let reloaded_config = load_create_config();
                    *self = BatteryStatus::Settings {
                        application_settings: reloaded_config,
                    };
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = match self {
            BatteryStatus::Main { battery_info } => row![battery_info.view()].width(Length::Fill),
            BatteryStatus::Settings {
                application_settings,
            } => row![
                TextInput::new("Battery Location", &application_settings.battery_location)
                    .on_input(Messages::InputNewBatteryLocation),
                button("Update").on_press(Messages::UpdateBatteryConfig)
            ]
            .width(Length::Fill),
        };

        let controls = match self {
            BatteryStatus::Main { battery_info: _ } => row![button("Settings")
                .padding(12)
                .on_press(Messages::OpenSettings)],
            BatteryStatus::Settings {
                application_settings: _,
            } => row![button("Back").padding(12).on_press(Messages::OpenMain)],
        };

        container(column!(controls, content).spacing(30).padding(10))
            .width(Length::Fill)
            .into()
    }
}

// fn system_theme_mode() -> Theme {
//     match dark_light::detect() {
//         dark_light::Mode::Light | dark_light::Mode::Default => Theme::Light,
//         dark_light::Mode::Dark => Theme::Dark,
//     }
// }

struct DataHelper {
    name: String,
    value: String,
}

#[derive(Default, Debug)]
pub struct BatteryInfo {
    native_path: String,
    vendor: String,
    model: String,
    serial: String,
    updated: String,
    has_statistics: bool,
    battery_present: bool,
    rechargeable: bool,
    warning_level: String,
    energy_now: f32,
    energy_empty: f32,
    energy_full: f32,
    energy_full_design: f32,
    percentage: f32,
    capacity: f32,
}

impl BatteryInfo {
    fn view(&self) -> Element<Messages> {
        let project_root = env!("CARGO_MANIFEST_DIR");

        let battery_image = match self.capacity {
            value if value > 75.0 => {
                let image_path = std::path::PathBuf::from(project_root)
                    .join("resources/batteryicons/battery_100.png");
                let image = image::Handle::from_path(image_path);
                image::viewer(image)
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(200.0))
                    .max_scale(10.0)
                    .min_scale(10.0)
            }
            value if value < 75.0 && value > 50.0 => {
                let image_path = std::path::PathBuf::from(project_root)
                    .join("resources/batteryicons/battery_75.png");
                let image = image::Handle::from_path(image_path);
                image::viewer(image)
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(200.0))
                    .max_scale(10.0)
                    .min_scale(10.0)
            }
            value if value < 50.0 && value > 25.0 => {
                let image_path = std::path::PathBuf::from(project_root)
                    .join("resources/batteryicons/battery_50.png");
                let image = image::Handle::from_path(image_path);
                image::viewer(image)
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(200.0))
                    .max_scale(10.0)
                    .min_scale(10.0)
            }
            _ => {
                let image_path = std::path::PathBuf::from(project_root)
                    .join("resources/batteryicons/battery_25.png");
                let image = image::Handle::from_path(image_path);
                image::viewer(image)
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(200.0))
                    .max_scale(10.0)
                    .min_scale(10.0)
            }
        };

        let battery_info_column = if self.battery_present {
            let battery_model_text = text(format!("Battery Model: {}", self.model)).size(20);
            let battery_state_text =
                text(format!("Current Battery: {}%", self.percentage)).size(20);
            let battery_energy_full_design = text(format!(
                "Designed Max Capacity: {}Wh",
                self.energy_full_design
            ))
            .size(20);
            let battery_energy_full =
                text(format!("Current Max Capacity: {}Wh", self.energy_full)).size(20);
            let battery_health = text(format!("Battery Health: {}%", self.capacity)).size(20);

            column!(
                battery_model_text,
                battery_state_text,
                battery_energy_full_design,
                battery_energy_full,
                battery_health
            )
        } else {
            column!(text(String::from(
                "Error, please check your battery location and check again"
            ))
            .style(Color::from_rgb(1.0, 0.0, 0.0))
            .size(20))
        };

        row!(battery_image, battery_info_column)
            .align_items(iced::Alignment::Center)
            .spacing(50.0)
            .into()
    }

    fn update_battery_info(battery_location: String) -> BatteryInfo {
        match Self::get_battery_stats(battery_location) {
            Ok(data) => Self::parse_datahelper(Self::parse_upower_ouput(&data)),
            Err(_err) => BatteryInfo::default(),
        }
    }

    fn get_battery_stats(battery_location: String) -> Result<String, String> {
        let output = std::process::Command::new("/bin/distrobox-host-exec")
            .arg("upower")
            .arg("-i")
            .arg(battery_location)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if output.status.success() {
            let result_str = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(result_str)
        } else {
            let error_str = String::from_utf8_lossy(&output.stderr).to_string();
            Err(format!("Command failed with error: {}", error_str))
        }
    }

    fn parse_upower_ouput(input: &str) -> Vec<DataHelper> {
        input
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.trim().splitn(2, ':').collect();
                if parts.len() == 2 {
                    Some(DataHelper {
                        name: parts[0].trim().to_string(),
                        value: parts[1].trim().to_string(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn parse_datahelper(data: Vec<DataHelper>) -> BatteryInfo {
        let mut battery_info = BatteryInfo::default();

        for d in data {
            match d.name.as_str().trim() {
                "native-path" => battery_info.native_path = d.value,
                "vendor" => battery_info.vendor = d.value,
                "model" => battery_info.model = d.value,
                "serial" => battery_info.serial = d.value,
                "updated" => battery_info.updated = d.value,
                "has statistics" => battery_info.has_statistics = d.value == "yes",
                "present" => battery_info.battery_present = d.value == "yes",
                "rechargeable" => battery_info.rechargeable = d.value == "yes",
                "warning-level" => battery_info.warning_level = d.value,
                "energy-now" => {
                    if let Some(value) = Self::extract_numeric(&d.value) {
                        battery_info.energy_now = value;
                    } else {
                        battery_info.energy_now = 0.0;
                    }
                }
                "energy-empty" => {
                    if let Some(value) = Self::extract_numeric(&d.value) {
                        battery_info.energy_empty = value;
                    } else {
                        battery_info.energy_empty = 0.0;
                    }
                }
                "energy-full" => {
                    if let Some(value) = Self::extract_numeric(&d.value) {
                        battery_info.energy_full = value;
                    } else {
                        battery_info.energy_full = 0.0;
                    }
                }
                "energy-full-design" => {
                    if let Some(value) = Self::extract_numeric(&d.value) {
                        battery_info.energy_full_design = value;
                    } else {
                        battery_info.energy_full_design = 0.0;
                    }
                }
                "percentage" => {
                    if let Some(value) = Self::extract_numeric(&d.value) {
                        battery_info.percentage = value;
                    } else {
                        battery_info.percentage = 0.0;
                    }
                }
                "capacity" => {
                    if let Some(value) = Self::extract_numeric(&d.value) {
                        battery_info.capacity = value;
                    } else {
                        battery_info.capacity = 0.0;
                    }
                }
                _ => (),
            }
        }
        battery_info
    }

    fn extract_numeric(value: &str) -> Option<f32> {
        // Remove percentage sign and then filter out non-digit characters
        let numeric_part: String = value
            .replace(',', ".")
            .replace('%', "")
            .chars()
            .filter(|&c| c.is_ascii_digit() || c == '.')
            .collect();
        numeric_part.parse::<f32>().ok()
    }
}
