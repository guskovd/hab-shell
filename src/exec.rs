extern crate habitat_core as hcore;
extern crate tempfile;

use std::path::{Path};
use std::process::{Command};
use std::os::unix::process::CommandExt;
use std::io::{Write};

pub fn exec(command: String, options: Vec<&str>) {
    let mut hart = hcore::package::PackageArchive::new(Path::new(super::PLAN_SH_LOCK));
    let hab_shell_command=". ./plan.sh; do_shell";
    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    write!(tmpfile, "{}", hab_shell_command).unwrap();
    let path = tmpfile.into_temp_path();
    let ident = hart.ident().unwrap();
    let mut exec_args = vec!("bash", "--rcfile", path.to_str().unwrap(), "-i");
    exec_args.extend(options);
    if !command.is_empty() {
        exec_args.extend(vec!("-c", &command))
    } else { // interactive shell
        println!("Welcome to Habitat Shell!");
    }
    
    Command::new("hab")
        .arg("pkg")
        .arg("exec")
        .arg(format!("{}/{}/{}/{}", ident.origin, ident.name, ident.version.unwrap(), ident.release.unwrap()))
        .args(exec_args)
        .exec();
}
