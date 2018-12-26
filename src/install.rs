extern crate habitat_core as hcore;

use common;
use std::path::{Path};
use std::process::{Command};

fn windows_install() {
    let mut inst = Command::new("hab")
        .arg("pkg")
        .arg("install")
        .arg(common::plan_lock_path())
        .spawn()
        .unwrap();
    inst.wait().unwrap();
}

fn linux_install() {
    let mut inst = Command::new("sudo")
        .arg("hab")
        .arg("pkg")
        .arg("install")
        .arg(common::plan_lock_path())
        .spawn()
        .unwrap();
    inst.wait().unwrap();
}

pub fn install_ident(ident: &hcore::package::PackageIdent) {
    let pkg_install = hcore::package::PackageInstall::load(&ident, Some(Path::new("/")));
    let _a = match pkg_install {
        Ok(_body) => {
            Ok(_body)
        },
        Err(_e)  => {
            if cfg!(target_os = "linux") {
                linux_install();
            } else if cfg!(target_os = "windows") {
                windows_install();
            } else {
                panic!("unsupported platform");
            }
            Err(_e)
        }
    };
}

pub fn install() {
    let ident = hcore::package::PackageArchive::new(common::plan_lock_path()).ident().unwrap();
    println!("installing {}/{}", ident.origin, ident.name);
    install_ident(&ident);
}
