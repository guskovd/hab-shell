extern crate clap;
extern crate dotenv;

use std::fs;
use std::env;
use clap::{Arg, App, SubCommand};
use std::os::unix::process::CommandExt;
use std::process::{Command};

static PLAN_SH_LOCK: &str = "plan.sh.lock";

fn pkg_artifact() -> String {
    format!("results/{}", env::var("pkg_artifact").unwrap())
}

fn load_env() {
    let _env = dotenv::from_filename("results/last_build.env");
}

fn lock() -> std::io::Result<()> {
    fs::copy(pkg_artifact(), PLAN_SH_LOCK)?;
    Ok(())
}

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
        if args.is_empty() {
            args = vec!("-R", ".");
        }
        println!("Building...");
        Command::new("hab")
            .arg("studio")
            .arg("build")
            .args(args)
            .exec();
        load_env();
        lock().unwrap();
    } else if let Some(_matches) = matches.subcommand_matches("install") {
        println!("Installing...");
        load_env();
        Command::new("sudo")
            .arg("hab")
            .arg("pkg")
            .arg("install")
            .arg(PLAN_SH_LOCK)
            .exec();
        
    } else if let Some(_matches) = matches.subcommand_matches("exec") {
        println!("Exec...");
    }

}

