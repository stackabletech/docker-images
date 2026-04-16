use clap::{Args, ValueEnum};

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
