use clap::{Args, Command, CommandFactory, ValueEnum};
use clap_complete::{
    Generator,
    Shell::{Bash, Elvish, Fish, Zsh},
};
use clap_complete_nushell::Nushell;

use crate::cli::Cli;

#[derive(Debug, Args)]
pub struct CompletionsArguments {
    /// Shell to generate completions for.
    #[arg(value_enum)]
    pub shell: Shell,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Shell {
    /// Bourne Again SHell
    Bash,

    /// Elvish shell
    Elvish,

    /// Friendly Interactive SHell
    Fish,

    /// Z SHell
    Zsh,

    /// Nushell
    Nushell,
}

/// This is the `boil completions` command handler function.
pub fn run_command(arguments: CompletionsArguments) {
    let mut cli = Cli::command();
    let bin_name = cli.get_bin_name().unwrap_or("boil").to_owned();

    match arguments.shell {
        Shell::Bash => generate(Bash, &mut cli, bin_name),
        Shell::Elvish => generate(Elvish, &mut cli, bin_name),
        Shell::Fish => generate(Fish, &mut cli, bin_name),
        Shell::Zsh => generate(Zsh, &mut cli, bin_name),
        Shell::Nushell => generate(Nushell, &mut cli, bin_name),
    }
}

fn generate(generator: impl Generator, cli: &mut Command, bin_name: String) {
    clap_complete::generate(generator, cli, bin_name, &mut std::io::stdout());
}
