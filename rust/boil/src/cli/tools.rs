use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct ToolsArguments {
    #[command(subcommand)]
    pub command: ToolsCommand,
}

#[derive(Debug, Subcommand)]
pub enum ToolsCommand {
    /// Turn the provided version tag into a floating tag according to boil's rules.
    //
    // The main user of this command is stackabletech/actions to be able to construct the floating
    // tag for the image index manifest tag. This command serves as a stepping stone for a more
    // robust solution using structured data output directly from boil.
    FloatingTag(FloatingTagArguments),
}

#[derive(Debug, Args)]
pub struct FloatingTagArguments {
    /// The source version tag which should be converted into a floating version tag.
    pub version: semver::Version,
}
