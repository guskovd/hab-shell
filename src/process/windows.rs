use std::process::{Command};

pub fn exec(cmd: &str, args: Vec<&str>) {
    Command::new(cmd).args(&args).status();
}
