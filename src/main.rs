extern crate clap;
extern crate dotenv;
extern crate habitat_core as hcore;
extern crate hab_shell;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("hab-shell")
        .version("1.0")
        .subcommand(SubCommand::with_name("install")
                    .about("hab pkg install wrapper")
                    .version("1.0")
        )
        .subcommand(SubCommand::with_name("exec")
                    .about("hab pkg exec wrapper")
                    .version("1.0")
                    .arg(Arg::with_name("command")
                         .short("c")
                         .long("command")
                         .takes_value(true)
                         .multiple(true)
                         .allow_hyphen_values(true)
                         .help("command")
                    )
                    .arg(Arg::with_name("options")
                         .help("shell options")
                         .short("o")
                         .long("options")
                         .takes_value(true)
                         .multiple(true)
                         .allow_hyphen_values(true)
                    )
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
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        let mut args = matches.values_of("args").unwrap_or_default().collect::<Vec<_>>();
        if args.is_empty() { args = vec!("-R", "."); }
        hab_shell::build::build(args);
    } else if let Some(_matches) = matches.subcommand_matches("install") {
        hab_shell::install::install();
    } else if let Some(matches) = matches.subcommand_matches("exec") {
        let mut options = matches.values_of("options").unwrap_or_default().collect::<Vec<_>>();
        let mut command = matches.values_of("command").unwrap_or_default().collect::<Vec<_>>().join(" ");
        hab_shell::exec::exec(command, options);
    }        
}

