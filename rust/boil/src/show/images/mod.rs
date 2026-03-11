use std::{collections::BTreeMap, io::IsTerminal};

use snafu::{ResultExt, Snafu};

use crate::{
    build::bakefile::{Targets, TargetsError, TargetsOptions},
    show::images::cli::{Pretty, ShowImagesArguments},
};

pub mod cli;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to serialize list as JSON"))]
    SerializeList { source: serde_json::Error },

    #[snafu(display("failed to build list of targets"))]
    BuildTargets { source: TargetsError },
}

/// This is the `boil show images` command handler function.
pub fn run_command(arguments: ShowImagesArguments) -> Result<(), Error> {
    let list: BTreeMap<_, _> = if arguments.image.is_empty() {
        Targets::all(TargetsOptions { only_entry: true })
            .context(BuildTargetsSnafu)?
            .into_iter()
    } else {
        Targets::set(&arguments.image, TargetsOptions { only_entry: true })
            .context(BuildTargetsSnafu)?
            .into_iter()
    }
    .map(|(image_name, image_versions)| {
        let versions: Vec<_> = image_versions
            .into_iter()
            .map(|(image_version, (_, _))| image_version)
            .collect();
        (image_name, versions)
    })
    .collect();

    print_to_stdout(list, arguments.pretty)
}

fn print_to_stdout(list: BTreeMap<String, Vec<String>>, pretty: Pretty) -> Result<(), Error> {
    let stdout = std::io::stdout();

    match pretty {
        Pretty::Always | Pretty::Auto if stdout.is_terminal() => {
            serde_json::to_writer_pretty(stdout, &list)
        }
        _ => serde_json::to_writer(stdout, &list),
    }
    .context(SerializeListSnafu)
}
