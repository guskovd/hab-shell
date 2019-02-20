extern crate clap;
extern crate dotenv;
extern crate habitat_core as hcore;

use clap::{Arg, App, SubCommand, AppSettings};

fn main() {
    let matches = App::new("hab-shell")
        .version("1.0")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("init")
                    .about("hab-shell init")
                    .version("1.0")
        )
        .subcommand(SubCommand::with_name("build")
                    .arg(Arg::with_name("args")
                         .help("optional hab pkg build args. (by default: '-R .')")
                         .multiple(true)
                         .allow_hyphen_values(true)
                         .last(true))
                    .about("hab pkg build wrapper")
                    .version("1.0")
        )
        .subcommand(SubCommand::with_name("install")
                    .about("hab pkg install wrapper")
                    .version("1.0")
        )
        .subcommand(SubCommand::with_name("freeze")
                    .about("hab pkg dependencies wrapper")
                    .arg(Arg::with_name("transitive")
                         .short("t")
                         .long("transitive")
                         .help("Show transitive dependencies")
                    )
                    .version("1.0")
        )
        .subcommand(SubCommand::with_name("run")
                    .about("hab pkg exec wrapper")
                    .version("1.0")
                    .arg(Arg::with_name("command")
                         .conflicts_with("script")
                         .short("c")
                         .long("command")
                         .takes_value(true)
                         .multiple(true)
                         .allow_hyphen_values(true)
                         .help("command")
                    )
                    .arg(Arg::with_name("script")
                         .conflicts_with("command")
                         .takes_value(true)
                         .help("script")
                    )
        )
        .subcommand(SubCommand::with_name("info")
                    .about("hab-shell info")
                    .subcommand(SubCommand::with_name("plan")
                                .about("Habitat Shell plan info")
                    )
                    .subcommand(SubCommand::with_name("lock")
                                .about("Habitat Shell plan lock info")
                    )
                    .version("1.0")
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("init") {
        hab_shell::init::init();
    } else if let Some(matches) = matches.subcommand_matches("build") {
        let mut args = matches.values_of("args").unwrap_or_default().collect::<Vec<_>>();
        if args.is_empty() { args = vec!("-R", "."); }
        hab_shell::build::build(args);
    } else if let Some(_matches) = matches.subcommand_matches("install") {
        hab_shell::install::install();
    } else if let Some(matches) = matches.subcommand_matches("freeze") {
        let mut is_transitive = false;
        if matches.occurrences_of("transitive") == 1 {
            is_transitive = true;
        }
        hab_shell::freeze::freeze(is_transitive);
    } else if let Some(matches) = matches.subcommand_matches("run") {
        let mut command = matches.values_of("command").unwrap_or_default().collect::<Vec<_>>().join(" ");
        let mut script = String::from(matches.value_of("script").unwrap_or_default());
        hab_shell::run::run(script, command);
    } else if let Some(matches) = matches.subcommand_matches("info") {
        hab_shell::info::info(matches);
    }        
}

