use std::{fs::File, io::Read, path::Path};

use crate::reloader::{Reloader, ReloaderOptions};
use clap::ArgMatches;
use notify::RecursiveMode;

use crate::cli::config::Config;
use crate::cli::init::DEFAULT_CONFIG_FILE;

pub fn handle_run(arg_matches: &ArgMatches) {
    let cfg_arg = arg_matches.get_one::<String>("cfg");

    let cfg_file = if let Some(cfg_path) = cfg_arg {
        File::open(Path::new(cfg_path))
    } else {
        File::open(Path::new(DEFAULT_CONFIG_FILE))
    };

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
