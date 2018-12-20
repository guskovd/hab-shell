extern crate habitat_core as hcore;

use std::path::{Path};
use std::process::{Command};

pub fn install_ident(ident: &hcore::package::PackageIdent) {
    let pkg_install = hcore::package::PackageInstall::load(&ident, Some(Path::new("/")));
    let _a = match pkg_install {
        Ok(_body) => {
            Ok(_body)
        },
        Err(_e)  => {
            let mut inst = Command::new("sudo")
                .arg("hab")
                .arg("pkg")
                .arg("install")
                .arg(super::PLAN_SH_LOCK)
                .spawn()
                .unwrap();
            inst.wait().unwrap();
            Err(_e)
        }
    };
}

pub fn install() {
    let ident = hcore::package::PackageArchive::new(Path::new(super::PLAN_SH_LOCK)).ident().unwrap();
    println!("installing {}/{}", ident.origin, ident.name);
    install_ident(&ident);
}
