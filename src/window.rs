use iced::{
    executor,
    widget::{column, container, image, row, text, Button, Image},
    Application, Color, Command, Length, Theme,
};

pub struct State {
    theme: Theme,
    battery_location: String,
    battery_status: BatteryInfo,
    errors: String,
}

#[derive(Debug, Clone)]
pub enum Messages {
    UpdateBatteryLocation(String),
    GetBatteryStatus,
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
                battery_location: String::from("/org/freedesktop/UPower/devices/battery_BAT0"), //TODO: Load config battery_location
                battery_status: {
                    match get_battery_stats(String::from(
                        "/org/freedesktop/UPower/devices/battery_BAT0", //TODO: Load config battery_location
                    )) {
                        Ok(data) => parse_datahelper(parse_upower_ouput(&data)),
                        Err(_err) => BatteryInfo::default(),
                    }
                },
                errors: String::from(""),
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
            Messages::GetBatteryStatus => {
                self.battery_status = match get_battery_stats(self.battery_location.to_string()) {
                    Ok(data) => parse_datahelper(parse_upower_ouput(&data)),
                    Err(err) => {
                        self.errors = err;
                        BatteryInfo::default()
                    }
                };
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let errors_output = text(self.errors.to_string());
        let project_root = env!("CARGO_MANIFEST_DIR");

        let battery_image = match self.battery_status.capacity {
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

        let battery_info_column =
            if self.errors.to_string().is_empty() && self.battery_status.battery_present {
                let battery_model_text =
                    text(format!("Battery Model: {}", self.battery_status.model)).size(20);
                let battery_state_text = text(format!(
                    "Current Battery: {}%",
                    self.battery_status.percentage
                ))
                .size(20);
                let battery_energy_full_design = text(format!(
                    "Designed Max Capacity: {}Wh",
                    self.battery_status.energy_full_design
                ))
                .size(20);
                let battery_energy_full = text(format!(
                    "Current Max Capacity: {}Wh",
                    self.battery_status.energy_full
                ))
                .size(20);
                let battery_health =
                    text(format!("Battery Health: {}%", self.battery_status.capacity)).size(20);

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

        let overall_battery_row = row!(battery_image, battery_info_column)
            .align_items(iced::Alignment::Center)
            .spacing(50.0);

        container(
            column!(errors_output, overall_battery_row)
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

struct DataHelper {
    name: String,
    value: String,
}

#[derive(Default)]
struct BatteryInfo {
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
                if let Some(value) = extract_numeric(&d.value) {
                    battery_info.energy_now = value;
                } else {
                    battery_info.energy_now = 0.0;
                }
            }
            "energy-empty" => {
                if let Some(value) = extract_numeric(&d.value) {
                    battery_info.energy_empty = value;
                } else {
                    battery_info.energy_empty = 0.0;
                }
            }
            "energy-full" => {
                if let Some(value) = extract_numeric(&d.value) {
                    battery_info.energy_full = value;
                } else {
                    battery_info.energy_full = 0.0;
                }
            }
            "energy-full-design" => {
                if let Some(value) = extract_numeric(&d.value) {
                    battery_info.energy_full_design = value;
                } else {
                    battery_info.energy_full_design = 0.0;
                }
            }
            "percentage" => {
                if let Some(value) = extract_numeric(&d.value) {
                    battery_info.percentage = value;
                } else {
                    battery_info.percentage = 0.0;
                }
            }
            "capacity" => {
                if let Some(value) = extract_numeric(&d.value) {
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
