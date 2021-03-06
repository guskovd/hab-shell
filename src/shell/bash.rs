use std::io::{Write};
use shell::shell::{Shell, Executor};

use {process,common};

pub struct Bash {
    shell: Shell
}

impl Executor for Bash {
    fn new() -> Bash {
        Bash { shell: Shell::new(
            "bash",
            &format!(". {}; do_shell", common::plan_path().to_str().unwrap()))
        }
    }

    fn exec(&self, script: String, command: String) {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "{}", self.shell.rc).unwrap();
        let rcfile = tmpfile.into_temp_path();
        let mut args = Shell::HAB_ARGS.to_vec();
        args.push(&self.shell.ident);
        args.push(&self.shell.interpreter);
        args.extend(vec!("--rcfile", rcfile.to_str().unwrap(), "-i"));
        if !command.is_empty() {
            args.extend(vec!("-c", &command))
        }
        if !script.is_empty() {
            args.push(&script);
        }
        process::exec(
            Shell::HAB_BIN,
            args
        );
    }
}
