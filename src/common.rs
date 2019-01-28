extern crate dirs;
extern crate dotenv;
extern crate ini;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::path::Path;
use self::ini::Ini;
use config::config;

pub fn project_root() -> PathBuf {
    let cur_dir = env::current_dir().unwrap();
    let path = Path::new(&cur_dir);
    for ancestor in path.ancestors() {
        let ances_manifest_path = ancestor.join(config::PLAN);
        if ances_manifest_path.exists() {
            return ancestor.to_path_buf();
        }
    }
    panic!("{} not found", config::PLAN)
}

pub fn plan_path() -> PathBuf {
    project_root().join(config::PLAN)
}

pub fn plan_lock_path() -> PathBuf {
    project_root().join(config::PLAN_LOCK)
}

pub fn get_home() -> PathBuf {
    let dir: PathBuf = match dirs::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
    };
    dir
}

pub fn pkg_artifact() -> String {
    format!("{}/results/{}", project_root().to_str().unwrap(), get_build_field("pkg_artifact"))
}

pub fn get_build_field(field: &str) -> String{
    let conf = Ini::load_from_file(format!("{}/results/{}", project_root().to_str().unwrap(), config::LAST_BUILD_ENV)).unwrap();
    let section = conf.general_section();
    let field_value = section.get(&format!("{}{}", config::LAST_BUILD_FIELD_PREFIX, field)).unwrap();
    field_value.to_string()
}

pub fn lock() -> std::io::Result<()> {
    fs::copy(pkg_artifact(), plan_lock_path())?;
    Ok(())
}
