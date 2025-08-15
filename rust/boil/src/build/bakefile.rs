use std::{
    collections::BTreeMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use glob::glob;
use serde::Serialize;
use snafu::{OptionExt, ResultExt, Snafu};
use time::format_description::well_known::Rfc3339;
use url::Host;

use crate::{
    VersionExt,
    build::{
        cli,
        docker::{BuildArgument, BuildArguments},
        image::{Image, ImageConfig, ImageConfigError, ImageOptions, VersionOptionsPair},
        platform::TargetPlatform,
    },
    config::Config,
    utils::{format_image_manifest_uri, format_image_repository_uri},
};

pub const OPEN_CONTAINER_IMAGE_REVISION: &str = "org.opencontainers.image.revision";
pub const OPEN_CONTAINER_IMAGE_CREATED: &str = "org.opencontainers.image.created";

pub const ENTRY_TARGET_NAME_PREFIX: &str = "entry--";

#[derive(Debug, Snafu)]
pub enum GitError {
    #[snafu(display("failed to open git repository"))]
    OpenRepository { source: git2::Error },

    #[snafu(display("failed to parse HEAD revision"))]
    ParseHeadRevision { source: git2::Error },

    #[snafu(display("failed to find starting point of rev range"))]
    InvalidRange,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to format current datetime"))]
    FormatTime { source: time::error::Format },

    #[snafu(display("failed to get revision"))]
    GetRevision { source: GitError },

    #[snafu(display("failed to create target graph"))]
    CreateGraph { source: TargetsError },
}

#[derive(Debug, Snafu)]
pub enum TargetsError {
    #[snafu(display("encountered invalid product version"))]
    InvalidProductVersion { source: ImageConfigError },

    #[snafu(display("failed to read image config"))]
    ReadImageConfig { source: ImageConfigError },
}

#[derive(Debug, Default)]
pub struct TargetsOptions {
    pub only_entry: bool,
}

/// Contains targets selected by the user.
///
/// This is a map which uses the image/target name as the key. Each key points to another map,
/// which contains one entry per version of the target. Each value contains the image options and
/// a boolean flag to indicate if this target is an entry target.
#[derive(Debug, Default)]
pub struct Targets(BTreeMap<String, BTreeMap<String, (ImageOptions, bool)>>);

impl Deref for Targets {
    type Target = BTreeMap<String, BTreeMap<String, (ImageOptions, bool)>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Targets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Targets {
    type Item = (String, BTreeMap<String, (ImageOptions, bool)>);
    type IntoIter =
        std::collections::btree_map::IntoIter<String, BTreeMap<String, (ImageOptions, bool)>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Targets {
    pub fn all(options: TargetsOptions) -> Result<Self, TargetsError> {
        let image_config_paths = glob("./**/boil-config.toml")
            .expect("glob pattern must be valid")
            .filter_map(Result::ok);

        let mut targets = Self::default();

        for image_config_path in image_config_paths {
            let image_config =
                ImageConfig::from_file(&image_config_path).context(ReadImageConfigSnafu)?;

            let image_name = image_config_path
                .parent()
                .expect("there must be a parent")
                .to_string_lossy()
                .into_owned();

            let pairs = image_config.all();

            targets.insert_targets(image_name.to_owned(), pairs, &options, true)?;
        }

        println!("{targets:#?}");

        Ok(targets)
    }

    pub fn from_images(images: &[Image], options: TargetsOptions) -> Result<Self, TargetsError> {
        let mut targets = Self::default();

        for image in images {
            // TODO (@Techassi): We should instead build the graph based on the Dockerfile(s),
            // because this is the source of truth and what ultimately gets built. The boil config
            // files are not a source a truth, but just provide data needed during the build.
            let image_config_path = PathBuf::new().join(&image.name).join("boil-config.toml");

            // Read the product config which defines supported product versions and their dependencies as
            // well as other values.
            let image_config =
                ImageConfig::from_file(image_config_path).context(ReadImageConfigSnafu)?;

            // Create a list of product versions we need to generate targets for in the bakefile.
            let pairs = image_config
                .filter_by_version(&image.versions)
                .context(InvalidProductVersionSnafu)?;

            targets.insert_targets(image.name.clone(), pairs, &options, true)?;
        }

        Ok(targets)
    }

    fn insert_targets(
        &mut self,
        image_name: String,
        pairs: Vec<VersionOptionsPair>,
        options: &TargetsOptions,
        is_entry: bool,
    ) -> Result<(), TargetsError> {
        for VersionOptionsPair {
            version: image_version,
            options: image_options,
        } in pairs
        {
            if !options.only_entry {
                // TODO (@Techassi): Add cycle detection
                for (image_name, image_version) in &image_options.local_images {
                    if self
                        .get(image_name)
                        .is_some_and(|image_versions| image_versions.contains_key(image_version))
                    {
                        continue;
                    }

                    let product_config_path =
                        PathBuf::new().join(image_name).join("boil-config.toml");

                    let product_config = ImageConfig::from_file(product_config_path)
                        .context(ReadImageConfigSnafu)?;

                    let pairs = product_config
                        .filter_by_version(&[image_version])
                        .context(InvalidProductVersionSnafu)?;

                    // Wowzers, recursion!
                    self.insert_targets(image_name.clone(), pairs, options, false)?;
                }
            }

            self.entry(image_name.clone())
                .or_default()
                .insert(image_version, (image_options, is_entry));
        }

        Ok(())
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Bakefile {
    #[serde(rename = "group")]
    pub groups: BTreeMap<String, BakefileGroup>,

    #[serde(rename = "target")]
    pub targets: BTreeMap<String, BakefileTarget>,
}

impl Bakefile {
    /// Create a bakefile from the [`BuildArguments`](cli::BuildArguments) provided via the CLI.
    ///
    /// This will only create targets for selected entry images and their dependencies. There is no
    /// need to filter anything out afterwards. The filtering is done automatically internally.
    pub fn from_args(args: &cli::BuildArguments, config: Config) -> Result<Self, Error> {
        let graph = Targets::from_images(&args.images, TargetsOptions::default())
            .context(CreateGraphSnafu)?;
        Self::from_targets(graph, args, config)
    }

    /// Returns all image manifest URIs for entry images.
    pub fn image_manifest_uris(&self) -> Vec<&str> {
        self.targets
            .iter()
            // We only care about the entry targets, because those are the primary images boil
            // builds.
            .filter(|(target_name, _)| target_name.starts_with(ENTRY_TARGET_NAME_PREFIX))
            // The image manifest URIs file only contains the image tags
            .flat_map(|(_, target)| &target.tags)
            // Flatten multiple tags (boil currently only ever writes a single one, but the data
            // structure can accept a list).
            .map(|s| s.as_str())
            .collect()
    }

    fn from_targets(
        targets: Targets,
        args: &cli::BuildArguments,
        config: Config,
    ) -> Result<Self, Error> {
        let mut bakefile_targets = BTreeMap::new();
        let mut groups: BTreeMap<String, BakefileGroup> = BTreeMap::new();

        // TODO (@Techassi): Can we somehow optimize this to come by with minimal amount of
        // cloning, because we also need to clone on every loop iteration below.
        let mut docker_build_arguments = config.build_arguments;
        docker_build_arguments.extend(args.docker_build_arguments.clone());
        docker_build_arguments.insert(BuildArgument::new(
            "RELEASE_VERSION".to_owned(),
            args.image_version.base_prerelease(),
        ));

        for (image_name, image_versions) in targets.0.into_iter() {
            for (image_version, (image_options, is_entry)) in image_versions {
                // TODO (@Techassi): Clean this up
                // TODO (@Techassi): Move the arg formatting into functions
                let mut docker_build_arguments = docker_build_arguments.clone();

                let local_version_docker_args: Vec<_> = image_options
                    .local_images
                    .iter()
                    .map(|(image_name, image_version)| {
                        BuildArgument::new(
                            format!(
                                "{image_name}_VERSION",
                                image_name = image_name.to_uppercase().replace('-', "_")
                            ),
                            image_version.to_string(),
                        )
                    })
                    .collect();

                docker_build_arguments.extend(image_options.build_arguments.clone());
                docker_build_arguments.extend(local_version_docker_args);
                docker_build_arguments.insert(BuildArgument::new(
                    "PRODUCT_VERSION".to_owned(),
                    image_version.to_string(),
                ));

                // The image registry, eg. `oci.stackable.tech` or `localhost`
                let image_registry = if args.use_localhost_registry {
                    &Host::Domain(String::from("localhost"))
                } else {
                    &args.registry
                };

                let image_repository_uri = format_image_repository_uri(
                    image_registry,
                    &args.registry_namespace,
                    &image_name,
                );

                let image_manifest_uri = format_image_manifest_uri(
                    &image_repository_uri,
                    &image_version,
                    &args.image_version,
                    args.target_platform.architecture(),
                );

                let dockerfile = PathBuf::new().join(&image_name).join("Dockerfile");
                let revision = Self::git_head_revision().context(GetRevisionSnafu)?;
                let date_time = Self::now()?;

                let target_name = if is_entry {
                    Self::format_entry_target_name(&image_name, &image_version)
                } else {
                    Self::format_target_name(&image_name, &image_version)
                };

                let contexts: BTreeMap<_, _> = image_options
                    .local_images
                    .iter()
                    .map(|(image_name, image_version)| {
                        let context_name = Self::format_context_name(image_name);
                        let context_target = Self::format_context_target(image_name, image_version);

                        (context_name, context_target)
                    })
                    .collect();

                let target = BakefileTarget {
                    annotations: BakefileTarget::annotations(&date_time, &revision),
                    labels: BakefileTarget::labels(date_time, revision),
                    tags: vec![image_manifest_uri],
                    arguments: docker_build_arguments,
                    platforms: vec![args.target_platform.clone()],
                    context: PathBuf::from("."),
                    dockerfile,
                    contexts,
                };

                bakefile_targets.insert(target_name, target);

                // Add the target to the default group if it is an entry
                if is_entry {
                    groups
                        .entry("default".to_owned())
                        .or_default()
                        .targets
                        .push(Self::format_entry_target_name(&image_name, &image_version));
                }
            }
        }

        Ok(Self {
            targets: bakefile_targets,
            groups,
        })
    }

    /// Formats and returns the entry target name, eg. `entry--opa-1_4_2`.
    fn format_entry_target_name(image_name: &str, image_version: &str) -> String {
        let target_name = Self::format_target_name(image_name, image_version);
        format!("{ENTRY_TARGET_NAME_PREFIX}{target_name}")
    }

    /// Formats and returns the target name, eg. `stackable-base-1_0_0`.
    fn format_target_name(image_name: &str, image_version: &str) -> String {
        // Replace any slashes from nested product names, eg. shared/protobuf, because docker buildx
        // has this weird restriction (because it also supports push, which we do on our own). We
        // are therefore artificially limited what target names we can use: [a-zA-Z0-9_-]+
        let product_name = image_name.replace('/', "__");

        // The dots in the semantic version also need to be replaced.
        let product_version = image_version.to_string().replace('.', "_");

        format!("{product_name}-{product_version}")
    }

    /// Formats and return the context name, eg. `stackable/image/stackable-base-1_0_0`.
    fn format_context_name(name: &str) -> String {
        format!("local-image/{name}")
    }

    /// Formats and returns the context target name, eg. `target:stackable-base-1_0_0`.
    fn format_context_target(name: &str, version: &str) -> String {
        let target_name = Self::format_target_name(name, version);
        format!("target:{target_name}")
    }

    fn now() -> Result<String, Error> {
        time::UtcDateTime::now()
            .format(&Rfc3339)
            .context(FormatTimeSnafu)
    }

    fn git_head_revision() -> Result<String, GitError> {
        let repo = git2::Repository::open(".").context(OpenRepositorySnafu)?;
        let rev = repo.revparse("HEAD").context(ParseHeadRevisionSnafu)?;
        let rev = rev.from().context(InvalidRangeSnafu)?.id().to_string();

        Ok(rev)
    }
}

// TODO (@Techassi): Figure out of we can use borrowed data in here. This would avoid a whole bunch
// of cloning.
#[derive(Debug, Serialize)]
pub struct BakefileTarget {
    pub annotations: Vec<String>,
    pub context: PathBuf,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub contexts: BTreeMap<String, String>,
    pub dockerfile: PathBuf,

    #[serde(rename = "args", skip_serializing_if = "BuildArguments::is_empty")]
    pub arguments: BuildArguments,

    pub labels: BTreeMap<String, String>,
    pub tags: Vec<String>,
    pub platforms: Vec<TargetPlatform>,
}

impl BakefileTarget {
    fn annotations(date_time: &str, revision: &str) -> Vec<String> {
        vec![
            format!("{OPEN_CONTAINER_IMAGE_CREATED}={date_time}"),
            format!("{OPEN_CONTAINER_IMAGE_REVISION}={revision}"),
        ]
    }

    fn labels(date_time: String, revision: String) -> BTreeMap<String, String> {
        BTreeMap::from([
            (OPEN_CONTAINER_IMAGE_CREATED.to_owned(), date_time.clone()),
            (OPEN_CONTAINER_IMAGE_REVISION.to_owned(), revision),
            ("build-date".to_owned(), date_time),
        ])
    }
}

#[derive(Debug, Default, Serialize)]
pub struct BakefileGroup {
    targets: Vec<String>,
}
