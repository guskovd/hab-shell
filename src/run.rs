extern crate habitat_core as hcore;
extern crate tempfile;

use {common, install, config};
use shell::{Bash,Shell, Pwsh};

pub fn run(command: String) {
    let ident = hcore::package::PackageArchive::new(common::plan_lock_path()).ident().unwrap();
    install::install_ident(&ident);

    if command.is_empty() {
        println!("Welcome to Habitat Shell!");
    }

    match config::config::PLATFORM {
        "unix" => Bash::new().exec(command),
        "windows" => Pwsh::new().exec(command),
        _ => panic!("unsupported platform!")
    };

}
