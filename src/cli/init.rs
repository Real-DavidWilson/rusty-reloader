use clap::ArgMatches;
use std::{fs::File, io::Write, path::Path};

pub const DEFAULT_CONFIG_FILE: &'static str = "rusty_reloader.json";

pub fn handle_init(_: &ArgMatches) {
    let file_path = Path::new(DEFAULT_CONFIG_FILE);

    let file = File::open(file_path);

    if file.is_ok() {
        println!("Configuration file already exists in this path.");

        return;
    }

    let mut file = File::create(file_path).expect(format!("Cannot open {file_path:?}").as_str());

    let initial_config_json = serde_json::to_string_pretty(&serde_json::json!({
        "cmd": "your initial command",
        "path": "your path to watch"
    }))
    .expect("Cannot convert initial configuration to json.");

    file.write(initial_config_json.as_bytes())
        .expect(format!("Error on write configuration to {file_path:?}").as_str());
}
