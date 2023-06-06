use clap::{arg, value_parser, Arg, ArgAction, ArgMatches, Command};

mod config;
mod init;
mod run;

fn create_cli() -> Command {
    Command::new("reloader")
        .about("Auto reloader written in rust.")
        .override_usage("rrld [OPTIONS] [COMMAND]")
        .subcommand(Command::new("init").arg(Arg::new("dir").index(1)))
        .arg(
            Arg::new("cfg")
                .long("config")
                .short('c')
                .help("Configuration file.")
                .global(false),
        )
        .allow_external_subcommands(true)
}

fn handle_matches(arg_matches: &ArgMatches) {
    let cfg_arg = arg_matches.get_one::<String>("cfg");

    run::handle_run(cfg_arg);
}

pub fn start() {
    let matches = create_cli().get_matches();

    match matches.subcommand() {
        Some(("init", arg_matches)) => {
            init::handle_init(arg_matches)
        }
        _ => handle_matches(&matches)
    }
}
