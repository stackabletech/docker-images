use std::collections::BTreeMap;

use snafu::{ResultExt, Snafu};

use crate::build::bakefile::{Targets, TargetsOptions};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to serialize list as JSON"))]
    SerializeList { source: serde_json::Error },
}

pub fn run_command() -> Result<(), Error> {
    let list: BTreeMap<_, _> = Targets::all(TargetsOptions { only_entry: true })
        .unwrap()
        .into_iter()
        .map(|(image_name, image_versions)| {
            let versions: Vec<_> = image_versions
                .into_iter()
                .map(|(image_version, (_, _))| image_version)
                .collect();
            (image_name, versions)
        })
        .collect();

    serde_json::to_writer_pretty(std::io::stdout(), &list).context(SerializeListSnafu)
}
