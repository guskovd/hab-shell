use std::process::{Command};
use std::os::unix::process::CommandExt;

pub fn install() {
    Command::new("sudo")
        .arg("hab")
        .arg("pkg")
        .arg("install")
        .arg(super::PLAN_SH_LOCK)
        .exec();
}
