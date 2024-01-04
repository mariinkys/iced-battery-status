use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct BatteryStatusConfiguration {
    pub battery_location: String,
}

pub fn load_create_config() -> BatteryStatusConfiguration {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let file_name = "config.json";
    let file_path = Path::new(project_root).join(file_name);

    // Check if the file exists
    if fs::metadata(&file_path).is_ok() {
        let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
        parse_json_config(contents)
    } else {
        match File::create(&file_path) {
            Ok(mut file) => {
                // You can write data to the file if needed
                writeln!(
                    file,
                    "{{\"battery_location\": \"/org/freedesktop/UPower/devices/battery_BAT0\"}}"
                )
                .expect("Failed to write to file");

                let contents =
                    fs::read_to_string(file_path).expect("Couldn't find or load that file.");
                parse_json_config(contents)
            }
            Err(_err) => BatteryStatusConfiguration::default(),
        }
    }
}

fn parse_json_config(json_content: String) -> BatteryStatusConfiguration {
    let p: BatteryStatusConfiguration =
        serde_json::from_str(&json_content).expect("Could not parse config");
    p
}

pub fn update_battery_config(json_content: String) {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let file_name = "config.json";
    let file_path = Path::new(project_root).join(file_name);

    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
    {
        // Write data to the file
        writeln!(file, "{}", json_content).expect("Failed to write to file");
    }
}
