use clap::CommandFactory;
use clap_complete::{Shell, generate};
use std::io;

/// Generate shell completion scripts for exkernel.
///
/// Outputs to stdout so users can redirect:
///   exkernel completions zsh > _exkernel
pub fn run(shell: Shell) {
    let mut cmd = crate::Cli::command();
    generate(shell, &mut cmd, "exkernel", &mut io::stdout());
}
