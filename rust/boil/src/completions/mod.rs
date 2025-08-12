use clap::CommandFactory;

use crate::cli::{Cli, CompletionsArguments};

pub fn run_command(arguments: CompletionsArguments) {
    let mut cli = Cli::command();
    let bin_name = cli.get_bin_name().unwrap_or("boil").to_owned();

    clap_complete::generate(arguments.shell, &mut cli, bin_name, &mut std::io::stdout());
}
