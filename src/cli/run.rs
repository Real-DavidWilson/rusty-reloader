use std::{fs::File, io::Read, path::Path};

use crate::reloader::{Reloader, ReloaderOptions};
use notify::RecursiveMode;

use crate::cli::config::Config;
use crate::cli::init::CONFIG_FILE;

pub fn handle_run(config_path: Option<&String>) {
    let cfg_file = File::open(Path::new(
        config_path.unwrap_or(&String::from(CONFIG_FILE)),
    ));

    if cfg_file.is_err() {
        panic!("Couldn't parse config file.");
    }

    let mut cfg_buf = String::new();

    cfg_file
        .unwrap()
        .read_to_string(&mut cfg_buf)
        .expect("Couldn't read config file.");

    let cfg: Config = serde_json::from_str(&*cfg_buf).expect("Couldn't parse config file.");

    let mut rld = Reloader::new(ReloaderOptions {
        cmd: &cfg.cmd,
        path: &cfg.path,
        recursive: Some(RecursiveMode::Recursive),
    });

    rld.watch();
}
