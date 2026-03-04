use clap::CommandFactory;
use clap_complete::{Shell, generate};
use std::io;

/// Generate shell completion scripts for existence.
///
/// Outputs to stdout so users can redirect:
///   existence completions zsh > _existence
pub fn run(shell: Shell) {
    let mut cmd = crate::Cli::command();
    generate(shell, &mut cmd, "existence", &mut io::stdout());
}
