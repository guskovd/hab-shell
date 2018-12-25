extern crate dirs;

use common;
use std::process::{Command};

fn unix_build_command(cache_path: String, root_dir: std::path::PathBuf, args: Vec<&str>) -> std::process::Child {
    Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("--privileged")
        .arg("-ti")
        .arg("-e").arg("HAB_NOCOLORING=true")
        .arg("-e").arg("HAB_ORIGIN=hab-shell")
        .arg("-v").arg("hab_pkgs:/hab/pkgs")
        .arg("-v").arg("hab_studios:/hab/studios")
        .arg("-v").arg(format!("{}:/hab/cache/keys", cache_path))
        .arg("-v").arg(format!("{}:/hab/cache/artifacts", cache_path))
        .arg("-v").arg(format!("{}:/src", root_dir.to_str().unwrap()))
        .arg("-v").arg("/var/run/docker.sock:/var/run/docker.sock")
        .arg("-w").arg("/src")
        .arg("dguskov/doha:base")
        .arg("hab")
        .arg("studio")
        .arg("build")
        .args(args)
        .current_dir(root_dir)
        .spawn()
        .unwrap()
}

fn windows_build_command(cache_path: String, root_dir: std::path::PathBuf, args: Vec<&str>) -> std::process::Child {
    Command::new("hab")
        .arg("hab")
        .arg("studio")
        .arg("build")
        .args(args)
        .env("HAB_ORIGIN", "hab-shell")
        .env("HAB_CACHE_KEY_PATH", cache_path)
        .current_dir(root_dir)
        .spawn()
        .unwrap()
}

pub fn build(args: Vec<&str>) {
    println!("Building...");
    super::init::init();
    let root_dir = common::project_root();
    let cache_path = format!("{}/.hab-shell-test/cache/keys", common::get_home().display());
    let mut build = Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("--privileged")
        .arg("-ti")
        .arg("-e").arg("HAB_NOCOLORING=true")
        .arg("-e").arg("HAB_ORIGIN=hab-shell")
        .arg("-v").arg("hab_pkgs:/hab/pkgs")
        .arg("-v").arg("hab_studios:/hab/studios")
        .arg("-v").arg(format!("{}:/hab/cache/keys", cache_path))
        .arg("-v").arg(format!("{}:/hab/cache/artifacts", cache_path))
        .arg("-v").arg(format!("{}:/src", root_dir.to_str().unwrap()))
        .arg("-v").arg("/var/run/docker.sock:/var/run/docker.sock")
        .arg("-w").arg("/src")
        .arg("dguskov/doha:base")
        .arg("hab")
        .arg("studio")
        .arg("build")
        .args(args)
        // .env("HAB_ORIGIN", "hab-shell")
        // .env("HAB_CACHE_KEY_PATH", cache_path)
        .current_dir(root_dir)
        .spawn()
        .unwrap();
    build.wait().unwrap();
    common::load_env();
    common::lock().unwrap();
}
