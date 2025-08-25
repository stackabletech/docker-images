use clap::{Args, CommandFactory};
use clap_complete::Shell;

use crate::cli::Cli;

#[derive(Debug, Args)]
pub struct CompletionsArguments {
    /// Shell to generate completions for.
    pub shell: Shell,
}

pub fn run_command(arguments: CompletionsArguments) {
    let mut cli = Cli::command();
    let bin_name = cli.get_bin_name().unwrap_or("boil").to_owned();

    clap_complete::generate(arguments.shell, &mut cli, bin_name, &mut std::io::stdout());
}
