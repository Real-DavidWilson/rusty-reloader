use clap::{Arg, ArgMatches, Command};

mod config;
mod init;
mod run;

fn create_cli() -> Command {
    Command::new("reloader")
        .about("Auto reloader written in rust.")
        .override_usage("rrld [OPTIONS] [COMMAND]")
        .arg(
            Arg::new("cfg")
                .long("config")
                .short('c')
                .help("Configuration file."),
        )
        .arg(
            Arg::new("init")
                .long("init")
                .help("Set up rust-reloader in the current path."),
        )
        .allow_external_subcommands(true)
}

fn handle_matches(arg_matches: &ArgMatches) {
    let init_arg = arg_matches.get_one::<String>("init");

    if init_arg.is_some() {
        init::handle_init(arg_matches);
        return;
    }

    run::handle_run(arg_matches);
}

pub fn start() {
    let matches = create_cli().get_matches();
    handle_matches(&matches);
}
