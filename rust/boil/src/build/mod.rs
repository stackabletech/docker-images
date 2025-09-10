use std::{
    fmt::Debug,
    process::{Command, Stdio},
};

use snafu::{OptionExt, ResultExt, Snafu, ensure};

use crate::{
    build::{bakefile::Bakefile, cli::BuildArguments},
    config::Config,
    utils::CommandExt,
};

pub mod bakefile;
pub mod cli;
pub mod docker;
pub mod image;
pub mod platform;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to create bakefile"))]
    CreateBakefile { source: bakefile::Error },

    #[snafu(display("failed to write image manifest URIs to file"))]
    WriteImageManifestUrisFile { source: std::io::Error },

    #[snafu(display("failed to serialize bakefile as JSON"))]
    SerializeBakefile { source: serde_json::Error },

    #[snafu(display("failed to acquire stdin handle"))]
    AcquireStdinHandle,

    #[snafu(display("failed to run child process"))]
    RunChildProcess { source: std::io::Error },

    #[snafu(display("failed to spawn child process"))]
    SpawnChildProcess { source: std::io::Error },

    #[snafu(display("encountered invalid image version, must not include any build metadata"))]
    InvalidImageVersion,
}

pub fn run_command(args: BuildArguments, config: Config) -> Result<(), Error> {
    // TODO (@Techassi): Parse Dockerfile instead to build the target graph
    // Validation
    ensure!(
        args.image_version.build.is_empty(),
        InvalidImageVersionSnafu
    );

    // Create bakefile
    let bakefile = Bakefile::from_args(&args, config).context(CreateBakefileSnafu)?;
    let image_manifest_uris = bakefile.image_manifest_uris();
    let count = image_manifest_uris.len();

    // Write the image manifest URIs to file if requested
    if let Some(path) = args.export_image_manifest_uris {
        std::fs::write(path, image_manifest_uris.join("\n"))
            .context(WriteImageManifestUrisFileSnafu)?;
    }

    // Output the bakefile contents if in dry-run mode
    if args.dry_run {
        return serde_json::to_writer_pretty(std::io::stdout(), &bakefile)
            .context(SerializeBakefileSnafu);
    }

    // TODO (@Techassi): Invoke this directly using the Docker daemon via bollard
    // or by building the image ourself.

    // Finally invoke the docker buildx bake command
    let mut child = Command::new("docker")
        .arg("buildx")
        .arg("bake")
        .arg_if(args.load, "--load")
        .arg("--file")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .context(SpawnChildProcessSnafu)?;

    // Acquire stdin handle to pipe the bakefile as JSON to it
    let stdin_handle = child.stdin.take().with_context(|| {
        child
            .kill()
            .expect("killing the child process must succeed");
        AcquireStdinHandleSnafu
    })?;

    serde_json::to_writer(stdin_handle, &bakefile).with_context(|_| {
        child
            .kill()
            .expect("killing the child process must succeed");
        SerializeBakefileSnafu
    })?;

    // Wait for successful completion of the child process
    let status = child.wait().context(RunChildProcessSnafu)?;

    // TODO (@Techassi): Return an error if the status was not a success
    if status.success() {
        println!(
            "Successfully built {count} image{plural}:\n{images}",
            plural = if count > 1 { "s" } else { "" },
            images = image_manifest_uris.join("\n")
        );
    }

    Ok(())
}
