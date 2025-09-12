use std::collections::BTreeMap;

use serde::{Serialize, ser::SerializeSeq};
use snafu::{ResultExt, Snafu};

use crate::{
    build::bakefile::{Targets, TargetsError, TargetsOptions},
    show::images::cli::ShowImagesArguments,
};

pub mod cli;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to serialize list as JSON"))]
    SerializeList { source: serde_json::Error },

    #[snafu(display("failed to build list of targets"))]
    BuildTargets { source: TargetsError },
}

// NOTE (@Techassi): I don't know if I like this... but this makes the stdout output very convient
// to consume.
struct OneOrMany(BTreeMap<String, Vec<String>>);

impl Serialize for OneOrMany {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.0.len() == 1 {
            let mut seq = serializer.serialize_seq(Some(1))?;
            for entry in &self.0 {
                for version in entry.1 {
                    seq.serialize_element(&version)?;
                }
            }

            Ok(seq.end()?)
        } else {
            self.0.serialize(serializer)
        }
    }
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

fn print_to_stdout(list: BTreeMap<String, Vec<String>>, pretty: bool) -> Result<(), Error> {
    let stdout = std::io::stdout();

    let list = OneOrMany(list);

    if pretty {
        serde_json::to_writer_pretty(stdout, &list).context(SerializeListSnafu)
    } else {
        serde_json::to_writer(stdout, &list).context(SerializeListSnafu)
    }
}
