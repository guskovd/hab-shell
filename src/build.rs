use std::process::{Command};

pub fn build(args: Vec<&str>) {
    println!("Building...");
    let mut build = Command::new("hab")
        .arg("studio")
        .arg("build")
        .args(args)
        .spawn()
        .unwrap();
    build.wait().unwrap();
    super::load_env();
    super::lock().unwrap();
}
