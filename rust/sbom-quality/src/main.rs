//! Checks the quality of the SBOMs that are attested to our published images.
//!
//! See the README for what is checked and why.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    process::ExitCode,
    sync::{
        Mutex, PoisonError,
        atomic::{AtomicUsize, Ordering},
    },
};

use clap::{Parser, ValueEnum};
use snafu::{ResultExt, Snafu, ensure};

use crate::{
    check::{Identity, ImageResult},
    expectations::{DEFAULT_PATH, ExpectationsFile, Target},
    registry::Attestation,
};

mod check;
mod expectations;
mod purl;
mod registry;
mod report;
mod sbom;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("failed to load the expectations"))]
    LoadExpectations { source: expectations::Error },

    #[snafu(display("failed to select the images to check"))]
    SelectTargets { source: expectations::Error },

    #[snafu(display(
        "local SBOM files need exactly one image (name=version or an operator, and --arch), got {count}"
    ))]
    LocalFilesNeedOneImage { count: usize },

    #[snafu(display("failed to read the SBOM file {path:?}"))]
    ReadSbom {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to write the JSON output"))]
    WriteJson { source: serde_json::Error },
}

/// Checks the SBOMs attested to the images in the expectations, every architecture, CycloneDX
/// and SPDX.
///
/// Needs `crane` and `cosign` on the PATH. Exits with 1 if there is an error finding and with 2 if
/// the check itself fails.
#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    /// Only check these products, product versions or operators: `kafka`, `kafka=4.3.1`.
    #[arg(value_name = "NAME[=VERSION]")]
    selectors: Vec<String>,

    /// The docker-images checkout, for the versions in the boil-config.toml files.
    #[arg(long, default_value = ".")]
    repository: PathBuf,

    /// The expectations file. Defaults to the one in the repository.
    #[arg(long)]
    expectations: Option<PathBuf>,

    /// Only check this architecture. Can be given more than once.
    #[arg(long = "arch")]
    architectures: Vec<String>,

    /// Keep fetched attestations in this directory. They are immutable per image digest.
    #[arg(long)]
    cache_dir: Option<PathBuf>,

    /// How many images are fetched and checked at the same time.
    #[arg(long, default_value_t = 8)]
    jobs: usize,

    #[arg(long, value_enum, default_value_t = Format::Text)]
    format: Format,

    /// List every finding and every image instead of a summary per check.
    #[arg(long)]
    verbose: bool,

    /// Print floors for the expectations from the current SBOMs instead of the findings.
    #[arg(long)]
    suggest_floors: bool,

    /// How far below the smallest current count the suggested floors are.
    #[arg(long, default_value_t = 0.2)]
    margin: f64,

    /// Check this CycloneDX file instead of the attestation in the registry.
    #[arg(long)]
    cyclonedx: Option<PathBuf>,

    /// Check this SPDX file instead of the attestation in the registry.
    #[arg(long)]
    spdx: Option<PathBuf>,

    /// The digest of the image the local files belong to.
    #[arg(long)]
    digest: Option<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Format {
    Text,
    Json,
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(exit_code) => exit_code,
        Err(error) => {
            eprintln!("{}", snafu::Report::from_error(error));
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<ExitCode, Error> {
    let expectations_path = cli
        .expectations
        .clone()
        .unwrap_or_else(|| cli.repository.join(DEFAULT_PATH));
    let expectations = ExpectationsFile::load(&expectations_path).context(LoadExpectationsSnafu)?;
    let architectures = if cli.architectures.is_empty() {
        expectations.architectures.clone()
    } else {
        cli.architectures.clone()
    };
    let (targets, problems) = expectations
        .select_targets(&cli.repository, &cli.selectors, &architectures)
        .context(SelectTargetsSnafu)?;

    let results = if cli.cyclonedx.is_some() || cli.spdx.is_some() {
        vec![check_local_files(&cli, &expectations, targets)?]
    } else {
        check_registry(&expectations, &targets, cli.cache_dir.as_deref(), cli.jobs)
    };

    if cli.suggest_floors {
        report::print_suggested_floors(&expectations, &results, cli.margin);
        return Ok(ExitCode::SUCCESS);
    }

    match cli.format {
        Format::Text => report::print_text(&results, &problems, cli.verbose),
        Format::Json => report::print_json(&results, &problems).context(WriteJsonSnafu)?,
    }

    if problems.is_empty() && !results.iter().any(ImageResult::has_errors) {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

fn check_local_files(
    cli: &Cli,
    expectations: &ExpectationsFile,
    targets: Vec<Target>,
) -> Result<ImageResult, Error> {
    ensure!(
        targets.len() == 1,
        LocalFilesNeedOneImageSnafu {
            count: targets.len()
        }
    );
    let read = |path: &Option<PathBuf>| {
        path.as_ref()
            .map(|path| std::fs::read_to_string(path).context(ReadSbomSnafu { path }))
            .transpose()
    };
    let cyclonedx = read(&cli.cyclonedx)?;
    let spdx = read(&cli.spdx)?;

    let Some(target) = targets.into_iter().next() else {
        return LocalFilesNeedOneImageSnafu { count: 0_usize }.fail();
    };
    let image_expectations = expectations.for_image(&target.image, target.version.as_deref());
    let mut result = ImageResult::new(target, cli.digest.clone());
    check::check_sboms(
        &mut result,
        cyclonedx.as_deref(),
        spdx.as_deref(),
        &image_expectations,
    );
    check::apply_ignores(&mut result, &image_expectations);
    Ok(result)
}

fn check_registry(
    expectations: &ExpectationsFile,
    targets: &[Target],
    cache_dir: Option<&Path>,
    jobs: usize,
) -> Vec<ImageResult> {
    let tags: Vec<(String, String)> = targets
        .iter()
        .map(|target| (target.image.clone(), target.tag.clone()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let digests: BTreeMap<&(String, String), _> = tags
        .iter()
        .zip(parallel_map(&tags, jobs, |(image, tag)| {
            registry::resolve_digests(&expectations.registry, image, tag)
        }))
        .collect();

    let checked = parallel_map(targets, jobs, |target| {
        let key = (target.image.clone(), target.tag.clone());
        check_target(expectations, target, digests.get(&key), cache_dir)
    });

    let mut identities = BTreeMap::new();
    let mut results = Vec::with_capacity(checked.len());
    for (result, identity) in checked {
        if let Some(identity) = identity {
            identities.insert(result.target.clone(), identity);
        }
        results.push(result);
    }

    check::check_architectures_agree(&mut results, &identities);
    for result in &mut results {
        let image_expectations =
            expectations.for_image(&result.target.image, result.target.version.as_deref());
        check::apply_ignores(result, &image_expectations);
    }
    results
}

/// Checks one image architecture. Also returns its components for comparing the architectures,
/// if it has any.
fn check_target(
    expectations: &ExpectationsFile,
    target: &Target,
    digests: Option<&Result<BTreeMap<String, String>, registry::Error>>,
    cache_dir: Option<&Path>,
) -> (ImageResult, Option<Identity>) {
    let mut result = ImageResult::new(target.clone(), None);
    let image_expectations = expectations.for_image(&target.image, target.version.as_deref());

    let digests = match digests {
        Some(Ok(digests)) => digests,
        Some(Err(error)) => {
            result.error(
                "image-missing",
                format!(
                    "cannot resolve {image}:{tag}: {error}",
                    image = target.image,
                    tag = target.tag,
                    error = error_chain(error),
                ),
            );
            return (result, None);
        }
        None => return (result, None),
    };
    let Some(digest) = digests.get(&target.architecture) else {
        result.error(
            "image-missing",
            format!(
                "{image}:{tag} has no {architecture} image",
                image = target.image,
                tag = target.tag,
                architecture = target.architecture,
            ),
        );
        return (result, None);
    };
    result.digest = Some(digest.clone());

    let mut fetch = |attestation: Attestation| match registry::fetch_sbom(
        expectations,
        &target.image,
        digest,
        attestation,
        cache_dir,
    ) {
        Ok(sbom) => Some(sbom),
        Err(error) => {
            result.error(
                attestation.check(),
                format!(
                    "no verified {} attestation: {}",
                    attestation.cosign_type(),
                    error_chain(&error)
                ),
            );
            None
        }
    };
    let cyclonedx = fetch(Attestation::CycloneDx);
    let spdx = fetch(Attestation::Spdx);

    let identity = check::check_sboms(
        &mut result,
        cyclonedx.as_deref(),
        spdx.as_deref(),
        &image_expectations,
    );
    (
        result,
        Some(identity).filter(|identity| !identity.is_empty()),
    )
}

/// An error and its sources on one line, for a finding.
fn error_chain(error: &dyn std::error::Error) -> String {
    std::iter::successors(Some(error), |error| error.source())
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(": ")
}

/// Maps the items on `jobs` threads, keeping their order.
fn parallel_map<T, R, F>(items: &[T], jobs: usize, function: F) -> Vec<R>
where
    T: Sync,
    R: Send,
    F: Fn(&T) -> R + Sync,
{
    let next = AtomicUsize::new(0);
    let results = Mutex::new(Vec::with_capacity(items.len()));
    std::thread::scope(|scope| {
        for _ in 0..jobs.clamp(1, items.len().max(1)) {
            scope.spawn(|| {
                loop {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some(item) = items.get(index) else {
                        break;
                    };
                    let result = function(item);
                    results
                        .lock()
                        .unwrap_or_else(PoisonError::into_inner)
                        .push((index, result));
                }
            });
        }
    });

    let mut results = results.into_inner().unwrap_or_else(PoisonError::into_inner);
    results.sort_by_key(|(index, _)| *index);
    results.into_iter().map(|(_, result)| result).collect()
}
