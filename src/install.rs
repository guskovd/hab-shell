extern crate habitat_core as hcore;

use std::path::{Path};
use std::process::{Command};
use std::os::unix::process::CommandExt;

pub fn install() {
    let mut hart = hcore::package::PackageArchive::new(Path::new(super::PLAN_SH_LOCK));
    let ident = hart.ident().unwrap();
    // let pkg_install = hcore::package::PackageInstall::load(&ident, Some(Path::new(".")));
    // println!("{:?}", pkg_install);
    Command::new("sudo")
        .arg("hab")
        .arg("pkg")
        .arg("install")
        .arg(super::PLAN_SH_LOCK)
        .exec();
}
