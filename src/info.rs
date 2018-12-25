extern crate clap;

use common;
use self::clap::{ArgMatches};

pub fn get_plan_path() -> String {
    let plan_path = common::plan_path();
    let plan = match plan_path.exists() {
        true => format!("{}", plan_path.to_str().unwrap()),
        false => format!("{} not found", common::PLAN_SH)
    };
    plan
}

pub fn get_plan_lock_path_or_panic() -> String {
    let plan_path = common::plan_lock_path();
    if !plan_path.exists() {
        panic!("{} not found!", common::PLAN_SH_LOCK)
    }
    return format!("{}", plan_path.to_str().unwrap());
}

pub fn get_plan_lock_path() -> String {
    let plan_path = common::plan_lock_path();
    let plan = match plan_path.exists() {
        true => format!("{}", plan_path.to_str().unwrap()),
        false => format!("{} not found", common::PLAN_SH_LOCK)
    };
    plan
}

pub fn info(matches: &ArgMatches) {
    if let Some(_matches) = matches.subcommand_matches("plan") {
        println!("{}", get_plan_path());
    } else  if let Some(_matches) = matches.subcommand_matches("lock") {
        println!("{}", get_plan_lock_path_or_panic());
    } else {
        println!("Hab-shell info:");
        println!("plan.sh: {}", get_plan_path());
        println!("plan.sh.lock: {}", get_plan_lock_path());
    }
}
