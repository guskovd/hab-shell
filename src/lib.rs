pub mod build;
pub mod install;
pub mod shell;
pub mod init;
pub mod freeze;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::path::Path;

static PLAN_SH: &str = "plan.sh";
static PLAN_SH_LOCK: &str = "plan.sh.lock";

fn project_root() -> PathBuf {
    let cur_dir = env::current_dir().unwrap();
    let path = Path::new(&cur_dir);
    for ancestor in path.ancestors() {
        let ances_manifest_path = ancestor.join(PLAN_SH);
        if ances_manifest_path.exists() {
            return ancestor.to_path_buf();
        }
    }
    panic!("{} not found", PLAN_SH)
}

fn plan_path() -> PathBuf {
    project_root().join(PLAN_SH)
}

fn plan_lock_path() -> PathBuf {
    project_root().join(PLAN_SH_LOCK)
}

fn get_home() -> PathBuf {
    let dir: PathBuf = match dirs::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
    };
    dir
}

fn pkg_artifact() -> String {
    format!("{}/results/{}", project_root().to_str().unwrap(), env::var("pkg_artifact").unwrap())
}

fn load_env() {
    let _env = dotenv::from_filename(format!("{}/results/last_build.env", project_root().to_str().unwrap()));
}

fn lock() -> std::io::Result<()> {
    fs::copy(pkg_artifact(), plan_lock_path())?;
    Ok(())
}
