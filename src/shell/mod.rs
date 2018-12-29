#[path = "shell.rs"]
pub mod shell;

#[path = "bash.rs"]
pub mod bash;

#[path = "pwsh.rs"]
pub mod pwsh;

pub use self::shell::{Shell,Executor};
pub use self::bash::Bash;
pub use self::pwsh::Pwsh;
