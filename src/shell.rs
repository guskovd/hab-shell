extern crate habitat_core as hcore;
extern crate tempfile;

use common;
use std::io::{Write};
use process;
use config::config;

fn linux_shell_args(rcfile: &str) -> Vec<&str>{
    vec!("bash", "--rcfile", rcfile, "-i")    
}

fn windows_shell_args(rcfile: &str) -> Vec<&str>{
    vec!("pwsh")    
}

pub fn shell(command: String, options: Vec<&str>) {
    let ident = hcore::package::PackageArchive::new(common::plan_lock_path()).ident().unwrap();
    super::install::install_ident(&ident);
    let hab_shell_command=format!(". {}; do_shell", common::plan_path().to_str().unwrap());
    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    write!(tmpfile, "{}", hab_shell_command).unwrap();
    let rcfile = tmpfile.into_temp_path();
    let mut shell_args = match config::PLATFORM {
        "unix" => linux_shell_args(rcfile.to_str().unwrap()),
        "windows" => windows_shell_args(rcfile.to_str().unwrap()),
        _ => panic!("unsupported platform!")
    };
    shell_args.extend(options);
    if !command.is_empty() {
        shell_args.extend(vec!("-c", &command))
    } else { // interactive shell
        println!("Welcome to Habitat Shell!");
    }

    let ident_str = format!("{}/{}/{}/{}", ident.origin, ident.name, ident.version.as_ref().unwrap(), ident.release.as_ref().unwrap());
    let mut args = vec!(
        "pkg",
        "exec",
        &ident_str
    );
    args.extend(shell_args);
    process::exec("hab", args);
}
