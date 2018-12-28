use std::io::{Write};
use shell::shell::{Shell, Executor};

use {process,common};

pub struct Bash {
    executor: Executor
}

impl Shell for Bash {
    fn new() -> Bash {
        Bash { executor: Executor::new(
            "bash",
            &format!(". {}; do_shell", common::plan_path().to_str().unwrap()))
        }
    }

    fn exec(&self, command: String) {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "{}", self.executor.rc).unwrap();
        let rcfile = tmpfile.into_temp_path();
        let mut args = Executor::HAB_ARGS.to_vec();
        args.push(&self.executor.ident);
        args.extend(vec!("bash", "--rcfile", rcfile.to_str().unwrap(), "-i"));
        if !command.is_empty() {
            args.extend(vec!("-c", &command))
        }
        process::exec(
            Executor::HAB_BIN,
            args
        );
    }
}
