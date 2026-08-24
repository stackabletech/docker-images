use std::{
    fmt::Debug,
    process::{Command, Stdio},
};

use snafu::{OptionExt, ResultExt, Snafu, ensure};

use crate::{
    cli::BuildArguments,
    config::Config,
    core::bakefile::{self, Bakefile},
    utils::CommandExt,
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to create bakefile"))]
    CreateBakefile { source: bakefile::Error },

    #[snafu(display("failed to create image manifest URIs file"))]
    CreateImageManifestUrisFile { source: std::io::Error },

    #[snafu(display("failed to serialize image manifest URIs file as JSON"))]
    SerializeImageManifestUrisFile { source: serde_json::Error },

    #[snafu(display("failed to serialize bakefile as JSON"))]
    SerializeBakefile { source: serde_json::Error },

    #[snafu(display("failed to acquire stdin handle"))]
    AcquireStdinHandle,

    #[snafu(display("failed to run child process"))]
    RunChildProcess { source: std::io::Error },

    #[snafu(display("failed to spawn child process"))]
    SpawnChildProcess { source: std::io::Error },

    #[snafu(display(
        "the docker child process failed with code {code}",
        code = code.map_or("unknown".to_owned(), |c| c.to_string())
    ))]
    ChildProcessFailed { code: Option<i32> },

    #[snafu(display("encountered invalid image version, must not include any build metadata"))]
    InvalidImageVersion,
}

/// This is the `boil build` command handler function.
pub fn run_command(args: Box<BuildArguments>, config: Config) -> Result<(), Error> {
    // TODO (@Techassi): Parse Dockerfile instead to build the target graph
    let bakefile = Bakefile::from_cli_args(&args, config).context(CreateBakefileSnafu)?;
    let image_manifest_uris = bakefile.image_manifest_uris();
    let count = image_manifest_uris
        .iter()
        .fold(0, |acc, (_, tags)| acc + tags.len());

    // Write the image manifest URIs to file if requested
    if let Some(path) = args.write_image_manifest_uris {
        let file = std::fs::File::create(path).context(CreateImageManifestUrisFileSnafu)?;
        serde_json::to_writer(file, &image_manifest_uris)
            .context(SerializeImageManifestUrisFileSnafu)?;
    }

    // Output the bakefile contents if in dry-run mode
    if args.dry_run {
        return serde_json::to_writer_pretty(std::io::stdout(), &bakefile)
            .context(SerializeBakefileSnafu);
    }

    // TODO (@Techassi): Invoke this directly using the Docker daemon via bollard
    // or by building the image ourself.

    // Finally invoke the docker buildx bake command
    #[allow(deprecated)]
    let mut child = Command::new("docker")
        .arg("buildx")
        .arg("bake")
        .arg_if(args.load, "--load")
        .args(args.rest)
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

    // Return an error if the child process failed
    ensure!(
        status.success(),
        ChildProcessFailedSnafu {
            code: status.code()
        }
    );

    // Take care of formatting output
    let mut built_images = String::new();
    for (name, tags) in image_manifest_uris {
        built_images.push_str(&format!("{name}:\n"));
        built_images.push_str(&format!(
            "  {tags}",
            tags = tags
                .iter()
                .map(|tags| tags.to_string())
                .collect::<Vec<String>>()
                .join("\n  ")
        ));
        built_images.push('\n');
    }

    print!(
        "Successfully built {count} image{plural}:\n{built_images}",
        plural = if count > 1 { "s" } else { "" },
    );

    Ok(())
}
