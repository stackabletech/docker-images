use crate::{cli::FloatingTagArguments, utils::VersionExt};

pub fn floating_tag(args: FloatingTagArguments) {
    let floating_tag = args.version.floating();
    println!("{floating_tag}");
}
