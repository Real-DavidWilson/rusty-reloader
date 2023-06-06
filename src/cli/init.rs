use std::{fs::File, io::Write, path::Path};

use clap::ArgMatches;

use crate::cli::config;

pub const CONFIG_FILE: &'static str = "rusty_reloader.json";

pub fn handle_init(arg_matches: &ArgMatches) {
    let dir = format!(
        "{}",
        arg_matches
            .get_one::<String>("dir")
            .unwrap_or(&String::from("."))
    );

    let init_path = Path::new(&dir);

    if init_path.is_file() {
        panic!("Only folder path is supported.");
    }

    let init_path = init_path.join(CONFIG_FILE);

    println!("{:?}", init_path);

    let file = File::open(&init_path);

    if file.is_ok() {
        println!("Configuration file already exists in this path.");

        return;
    }

    let mut file = File::create(&init_path).expect(format!("Cannot open {init_path:?}").as_str());

    let config = config::Config {
        cmd: String::from("your initial command"),
        path: String::from("your path to watch"),
        delay: 0,
    };

    let initial_config_json = serde_json::to_string_pretty(&config)
        .expect("Cannot convert initial configuration to json.");

    file.write(initial_config_json.as_bytes())
        .expect(format!("Error on write configuration to {init_path:?}").as_str());
}
