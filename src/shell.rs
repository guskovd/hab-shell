extern crate habitat_core as hcore;
extern crate tempfile;

use common;
use std::process::{Command};
// use std::os::unix::process::CommandExt;
use std::io::{Write};

pub fn shell(command: String, options: Vec<&str>) {
    let ident = hcore::package::PackageArchive::new(common::plan_lock_path()).ident().unwrap();
    super::install::install_ident(&ident);
    let hab_shell_command=format!(". {}; do_shell", common::plan_path().to_str().unwrap());
    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    write!(tmpfile, "{}", hab_shell_command).unwrap();
    let path = tmpfile.into_temp_path();
    let mut shell_args = vec!("bash", "--rcfile", path.to_str().unwrap(), "-i");
    shell_args.extend(options);
    if !command.is_empty() {
        shell_args.extend(vec!("-c", &command))
    } else { // interactive shell
        println!("Welcome to Habitat Shell!");
    }
    
    // Command::new("hab")
    //     .arg("pkg")
    //     .arg("exec")
    //     .arg(format!("{}/{}/{}/{}", ident.origin, ident.name, ident.version.unwrap(), ident.release.unwrap()))
    //     .args(shell_args)
    //     .exec();
}
