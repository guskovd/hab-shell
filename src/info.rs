use common;

pub fn get_plan_path() -> String {
    let plan_path = common::plan_path();
    let plan = match plan_path.exists() {
        true => format!("{}", plan_path.to_str().unwrap()),
        false => format!("plan.sh not found")
    };
    plan
}

pub fn get_plan_lock_path() -> String {
    let plan_path = common::plan_lock_path();
    let plan = match plan_path.exists() {
        true => format!("{}", plan_path.to_str().unwrap()),
        false => format!("plan.sh.lock not found")
    };
    plan
}


pub fn info() {
    println!("Hab-shell info:");
    println!("plan.sh: {}", get_plan_path());
    println!("plan.sh.lock: {}", get_plan_lock_path());
}
