extern crate habitat_core as hcore;
extern crate tempfile;

use {common, install, config};
use shell::{Bash,Pwsh, Executor};

pub fn run(script: String, command: String) {
    let ident = hcore::package::PackageArchive::new(common::plan_lock_path()).ident().unwrap();
    install::install_ident(&ident);

    if command.is_empty() & script.is_empty() {
        println!("Welcome to Habitat Shell!");
    }

    match config::config::PLATFORM {
        "unix" => Bash::new().exec(script, command),
        "windows" => Pwsh::new().exec(script, command),
        _ => panic!("unsupported platform!")
    };

}
