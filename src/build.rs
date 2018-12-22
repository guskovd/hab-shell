extern crate dirs;

use std::process::{Command};

pub fn build(args: Vec<&str>) {
    println!("Building...");
    super::init::init();
    let root_dir = super::project_root();
    let cache_path = format!("{}/.hab-shell-test/cache/keys", super::get_home().display());
    let mut build = Command::new("hab")
        .arg("studio")
        .arg("build")
        .args(args)
        .env("HAB_ORIGIN", "hab-shell")
        .env("HAB_CACHE_KEY_PATH", cache_path)
        .current_dir(root_dir)
        .spawn()
        .unwrap();
    build.wait().unwrap();
    super::load_env();
    super::lock().unwrap();
}
