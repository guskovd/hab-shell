extern crate tempfile;
extern crate habitat_core as hcore;

use {common};

pub struct Shell {
    pub ident: String,
    pub interpreter: String,
    pub rc: String
}

impl Shell {
    pub const HAB_BIN: &'static str = "hab";
    pub const HAB_ARGS: [&'static str; 2] = ["pkg", "exec"];
    pub fn new(interpeter: &str, rc: &str) -> Shell {
        let ident = hcore::package::PackageArchive::new(common::plan_lock_path()).ident().unwrap();
        let ident_str = format!("{}/{}/{}/{}", ident.origin, ident.name, ident.version.as_ref().unwrap(), ident.release.as_ref().unwrap());
        Shell {
            ident: ident_str,
            interpreter: String::from(interpeter),
            rc: String::from(rc)
        }
    }
}


pub trait Executor {
    fn new() -> Self;
    fn exec(&self, command: String);
}


