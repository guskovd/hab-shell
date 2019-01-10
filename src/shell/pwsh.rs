use shell::shell::{Shell, Executor};

use {process,common};

pub struct Pwsh {
    shell: Shell
}

impl Executor for Pwsh {
    fn new() -> Pwsh {
        Pwsh { shell: Shell::new(
            "pwsh",
            &format!(". {}; Invoke-Shell", common::plan_path().to_str().unwrap()))
        }
    }
    
    fn exec(&self, _script: String, _command: String) {
        let mut args = Shell::HAB_ARGS.to_vec();
        args.push(&self.shell.ident);
        args.push(&self.shell.interpreter);
        args.extend(vec!("-NoExit", "-c", &self.shell.rc));
        process::exec(
            Shell::HAB_BIN,
            args
        );
    }
}
