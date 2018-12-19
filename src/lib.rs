pub mod build;
pub mod install;
pub mod shell;
pub mod init;

use std::env;
use std::fs;

static PLAN_SH_LOCK: &str = "plan.sh.lock";

fn pkg_artifact() -> String {
    format!("results/{}", env::var("pkg_artifact").unwrap())
}

fn load_env() {
    let _env = dotenv::from_filename("results/last_build.env");
}

fn lock() -> std::io::Result<()> {
    fs::copy(pkg_artifact(), PLAN_SH_LOCK)?;
    Ok(())
}
