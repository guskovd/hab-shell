use shell::shell::{Shell, Executor};

use {process,common};

pub struct Pwsh {
    executor: Executor
}

impl Shell for Pwsh {
    fn new() -> Pwsh {
        Pwsh { executor: Executor::new(
            "pwsh",
            &format!(". {}; Invoke-Shell", common::plan_path().to_str().unwrap()))
        }
    }
    
    fn exec(&self, _command: String) {
        let mut args = Executor::HAB_ARGS.to_vec();
        args.push(&self.executor.ident);
        args.extend(vec!("-NoExit", "-c", &self.executor.rc));
        process::exec(
            Executor::HAB_BIN,
            args
        );
    }
}
