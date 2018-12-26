use std::process::{Command};
use std::os::unix::process::CommandExt;

pub fn exec(cmd: &str, args: Vec<&str>) {
    Command::new(cmd)
        .args(args)
        .exec();
}
