extern crate habitat_core as hcore;

use std::path::{Path};

pub fn freeze(is_transitive: bool) {
    let ident = hcore::package::PackageArchive::new(Path::new(super::PLAN_SH_LOCK)).ident().unwrap();
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
