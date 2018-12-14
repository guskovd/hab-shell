extern crate clap;

use clap::{Arg, App, SubCommand};
use std::os::unix::process::CommandExt;
use std::process::{Command};

fn main() {
    let matches = App::new("hab-shell")
        .version("1.0")
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("install")
                    .about("hab pkg install wrapper")
                    .version("1.0")
                    .help("build pkg")
        )
        .subcommand(SubCommand::with_name("build")
                    .arg(Arg::with_name("args")
                         .multiple(true)
                         .allow_hyphen_values(true)
                         .last(true))
                    .about("hab pkg build wrapper")
                    .version("1.0")
                    .help("build pkg")
        )
        .get_matches();

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }


    if let Some(matches) = matches.subcommand_matches("build") {
        let mut args = matches.values_of("args").unwrap_or_default().collect::<Vec<_>>();
        if args.is_empty() {
            args = vec!("-R", ".");
        }
        println!("Building...");
        Command::new("hab")
            .arg("studio")
            .arg("build")
            .args(args)
            .exec();
    } else if let Some(_matches) = matches.subcommand_matches("install") {
        println!("Installing...");
    }

}

