extern crate habitat_core as hcore;

use common;
use std::path::{Path};

pub fn freeze(is_transitive: bool) {
    let ident = hcore::package::PackageArchive::new(common::plan_lock_path()).ident().unwrap();
    let pkg_install = hcore::package::PackageInstall::load(&ident, Some(Path::new("/")));
    if is_transitive {
        let deps = pkg_install.unwrap().tdeps().unwrap();
        for dep in deps.iter() {
            println!("{}/{}/{}/{}", dep.origin, dep.name, dep.version.as_ref().unwrap(), dep.release.as_ref().unwrap());
        }
    } else {
        let deps = pkg_install.unwrap().deps().unwrap();
        for dep in deps.iter() {
            println!("{}/{}/{}/{}", dep.origin, dep.name, dep.version.as_ref().unwrap(), dep.release.as_ref().unwrap());
        }
    }
}
