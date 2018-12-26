#[cfg(unix)]
pub mod config {
    pub static PLATFORM: &str = "unix";
    pub static PLAN: &str = "plan.sh";
    pub static PLAN_LOCK: &str = "plan.sh.lock";
    pub static LAST_BUILD_ENV: &str = "last_build.env";
    pub static LAST_BUILD_FIELD_PREFIX: &str = "";
}

#[cfg(windows)]
pub mod config {
    pub static PLATFORM: &str = "windows";
    pub static PLAN: &str = "plan.ps1";
    pub static PLAN_LOCK: &str = "plan.ps1.lock";
    pub static LAST_BUILD_ENV: &str = "last_build.ps1";
    pub static LAST_BUILD_FIELD_PREFIX: &str = "$";
}
