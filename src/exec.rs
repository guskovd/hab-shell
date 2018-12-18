extern crate habitat_core as hcore;

use std::path::{Path};
use std::process::{Command};
use std::os::unix::process::CommandExt;

pub fn exec(args: Vec<&str>) {
    let mut hart = hcore::package::PackageArchive::new(Path::new(super::PLAN_SH_LOCK));
    let ident = hart.ident().unwrap();
    let mut exec_args = vec!("bash", "--rcfile", ".bashrc");
    exec_args.extend(args);
    println!("Welcome to Habitat Shell!");
    Command::new("hab")
        .arg("pkg")
        .arg("exec")
        .arg(format!("{}/{}/{}/{}", ident.origin, ident.name, ident.version.unwrap(), ident.release.unwrap()))
        .args(exec_args)
        .exec();
}
