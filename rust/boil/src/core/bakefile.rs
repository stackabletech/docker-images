use std::{
    collections::{BTreeMap, btree_map::Entry},
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use cap_std::{ambient_authority, fs::Dir};
use glob::glob;
use oci_spec::image::{
    ANNOTATION_AUTHORS, ANNOTATION_CREATED, ANNOTATION_DOCUMENTATION, ANNOTATION_LICENSES,
    ANNOTATION_REVISION, ANNOTATION_SOURCE, ANNOTATION_VENDOR, ANNOTATION_VERSION,
};
use serde::{Serialize, ser::SerializeSeq};
use snafu::{OptionExt, ResultExt, Snafu, ensure};
use time::format_description::well_known::Rfc3339;

use crate::{
    cli::{self, HostPort},
    config::{self, Config, MetadataOptions},
    constants::DOCKER_LABEL_BUILD_DATE,
    core::{
        docker,
        image::{ImageConfig, ImageConfigError, ImageSelector},
        platform::TargetPlatform,
    },
    utils,
};

pub const COMMON_TARGET_NAME: &str = "common--target";
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

    #[snafu(display("failed to parse build arguments"))]
    ParseBuildArguments {
        source: docker::ParseBuildArgumentsError,
    },

    #[snafu(display("failed to locate containerfile relative to the {path:?} directory"))]
    NoSuchContainerfileExists { path: String },

    #[snafu(display("failed to open scoped directory as {path}"))]
    OpenScopedDirectory {
        source: std::io::Error,
        path: String,
    },

    #[snafu(display("failed to parse floating vendor version"))]
    ParseFloatingVendorVersion {
        source: utils::ParseFloatingVendorVersionError,
    },
}

#[derive(Debug, Snafu)]
pub enum TargetsError {
    #[snafu(display("failed to read image config"))]
    ReadImageConfig { source: ImageConfigError },

    #[snafu(display("failed to resolve parent directory of image config at {path}", path = path.display()))]
    ResolveParentDirectory { path: PathBuf },

    #[snafu(display("provided filter version(s) ({image_name}={versions}) yielded empty list", versions = versions.join(", ")))]
    EmptyFilter {
        versions: Vec<String>,
        image_name: String,
    },

    #[snafu(display(
        "failed to resolve local image chain. Circular dependency found: {}",
        chain.iter().map(|(n, v)| format!("{n}={v}")).collect::<Vec<_>>().join(" -> ")
    ))]
    CircularDependency { chain: Vec<(String, String)> },
}

#[derive(Debug, Default)]
pub struct TargetsOptions {
    /// Only select the entry images (selected by the image selector). This is particular useful for
    /// the image list command.
    pub only_entry: bool,

    /// If a non recursive glob pattern should be used meaning only the top-level directories will
    /// be searched for config files.
    pub non_recursive: bool,
}

/// Contains targets selected by the user.
///
/// This is a map which uses the image/target name as the key. Each key points to image config
/// containing filtered versions. Additionally, each value contains a boolean flag to indicate if
/// this target is an entry target.
#[derive(Debug, Default)]
pub struct Targets(BTreeMap<String, (ImageConfig, bool)>);

impl Deref for Targets {
    type Target = BTreeMap<String, (ImageConfig, bool)>;

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
    type IntoIter = std::collections::btree_map::IntoIter<String, (ImageConfig, bool)>;
    type Item = (String, (ImageConfig, bool));

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Targets {
    /// Returns a map of all targets by globbing for (nested) image config files.
    ///
    /// The search behaviour can be customized using the provided [`TargetsOptions`].
    //
    // SAFETY: We purposefully allow the `clippy::unwrap_in_result` lint below in this function.
    // We can use expect here, because the glob pattern is defined as a constant and the glob
    // function only returns an error if the pattern is invalid. We must ensure the pattern is
    // valid at compile time, because there is no need to allow an invalid pattern which would
    // render this tool inoperable.
    //
    // FIXME (@Techassi): This attribute can be used on individual unwrap and expect calls since
    // Rust 1.91.0. We should move this attribute to not contaminate an unnecessarily large scope
    // once we bump the toolchain to 1.91.0.
    // See https://github.com/rust-lang/rust-clippy/pull/15445
    #[allow(clippy::unwrap_in_result)]
    pub fn all(options: TargetsOptions) -> Result<Self, TargetsError> {
        let image_config_paths = if options.non_recursive {
            glob(ImageConfig::FLAT_CONFIG_GLOB_PATTERN)
        } else {
            glob(ImageConfig::ALL_CONFIGS_GLOB_PATTERN)
        }
        .expect("constant glob pattern must be valid")
        .filter_map(Result::ok);

        let mut targets = Self::default();

        for image_config_path in image_config_paths {
            let image_config =
                ImageConfig::from_file(&image_config_path).context(ReadImageConfigSnafu)?;

            let image_name = image_config_path
                .parent()
                .with_context(|| ResolveParentDirectorySnafu {
                    path: image_config_path.clone(),
                })?
                .to_string_lossy()
                .into_owned();

            targets.insert_targets(
                image_name.to_owned(),
                image_config,
                &options,
                true,
                &mut Vec::new(),
            )?;
        }

        Ok(targets)
    }

    /// Returns a filtered set out of all targets by looking up selected image config files.
    ///
    /// The search behaviour can be customized using the provided [`TargetsOptions`].
    pub fn set(images: &[ImageSelector], options: TargetsOptions) -> Result<Self, TargetsError> {
        let mut targets = Self::default();

        for image in images {
            // TODO (@Techassi): We should instead build the graph based on the Dockerfile(s),
            // because this is the source of truth and what ultimately gets built. The boil config
            // files are not a source a truth, but just provide data needed during the build.
            let image_config_path = PathBuf::new()
                .join(&image.name)
                .join(ImageConfig::DEFAULT_FILE_NAME);

            // Read the image config which defines supported image versions and their dependencies as
            // well as other values.
            let mut image_config =
                ImageConfig::from_file(image_config_path).context(ReadImageConfigSnafu)?;

            // Create a list of image versions we need to generate targets for in the bakefile.
            image_config.filter_by_version(&image.versions);

            ensure!(
                !image_config.versions.is_empty(),
                EmptyFilterSnafu {
                    versions: image.versions.clone(),
                    image_name: image.name.clone(),
                }
            );

            targets.insert_targets(
                image.name.clone(),
                image_config,
                &options,
                true,
                &mut Vec::new(),
            )?;
        }

        Ok(targets)
    }

    fn insert_targets(
        &mut self,
        image_name: String,
        config: ImageConfig,
        options: &TargetsOptions,
        is_entry: bool,
        chain: &mut Vec<(String, String)>,
    ) -> Result<(), TargetsError> {
        for (version, image_options) in (*config.versions).iter() {
            if !options.only_entry {
                let dependency = (image_name.clone(), version.clone());

                // If the current image name and image version combination is already in the current
                // dependency chain, we hit a circular dependency and abort immediately.
                ensure!(
                    !chain.contains(&dependency),
                    CircularDependencySnafu {
                        chain: chain
                            .iter()
                            .cloned()
                            .chain(std::iter::once(dependency))
                            .collect::<Vec<_>>(),
                    }
                );
                chain.push(dependency);

                for (image_name, image_version) in &image_options.local_images {
                    if self
                        .get(image_name)
                        .is_some_and(|(config, _)| config.versions.contains_key(image_version))
                    {
                        continue;
                    }

                    let image_config_path = PathBuf::new()
                        .join(image_name)
                        .join(ImageConfig::DEFAULT_FILE_NAME);

                    let mut image_config =
                        ImageConfig::from_file(image_config_path).context(ReadImageConfigSnafu)?;

                    image_config.filter_by_version(&[image_version]);

                    ensure!(
                        !image_config.versions.is_empty(),
                        EmptyFilterSnafu {
                            versions: vec![image_version.clone()],
                            image_name: image_name.clone(),
                        }
                    );

                    // Wowzers, recursion!
                    self.insert_targets(image_name.clone(), image_config, options, false, chain)?;
                }

                // Remove the last dependency as soon as we are done looking at that particular
                // dependency (name+version combination). We do this because we are not decending
                // down the chain for this particular dependency anymore, but instead move to the
                // next dependency at the same level of depth. Illustration:
                //
                // foo
                //   bar
                //     (no further deps, pop "bar")
                //   baz
                chain.pop();
            }
        }

        // We explicitly use the entry API without using the combinator functions because of issues
        // regarding partial moves and borrowing.
        match self.entry(image_name) {
            Entry::Vacant(entry) => {
                entry.insert((config, is_entry));
            }
            Entry::Occupied(mut entry) => {
                let (exiting_config, _) = entry.get_mut();
                exiting_config.versions.extend(config.versions);
            }
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
    pub fn from_cli_args(cli_args: &cli::BuildArguments, config: Config) -> Result<Self, Error> {
        let targets =
            Targets::set(&cli_args.images, TargetsOptions::default()).context(CreateGraphSnafu)?;
        Self::from_targets(targets, cli_args, config)
    }

    /// Returns all image manifest URIs for entry images.
    pub fn image_manifest_uris(&self) -> BTreeMap<&str, Vec<&TagSet>> {
        self.targets
            .iter()
            // We only care about the entry targets, because those are the primary images boil
            // builds.
            .filter(|(target_name, _)| target_name.starts_with(ENTRY_TARGET_NAME_PREFIX))
            // The image manifest URIs file only contains the image tags
            // Group tags by image name and collect them into a map of tag sets
            .fold(BTreeMap::new(), |mut acc, (_, target)| {
                acc.entry(target.image_name.as_str())
                    .and_modify(|tags| tags.push(&target.tags))
                    .or_insert(vec![&target.tags]);
                acc
            })
    }

    /// Creates the common target, containing shared data, which will be inherited by other targets.
    fn common_target(
        cli_args: &cli::BuildArguments,
        container_build_args: docker::BuildArguments,
        metadata: &MetadataOptions,
    ) -> Result<BakefileTarget, Error> {
        let revision = Self::git_head_revision().context(GetRevisionSnafu)?;
        let date_time = Self::now()?;

        // Load build arguments from a file if the user requested it
        let mut user_container_build_args: docker::BuildArguments =
            docker::build_args_vec_to_btree_map(cli_args.build_arguments.clone());

        if let Some(path) = &cli_args.build_arguments_file {
            let build_arguments_from_file =
                docker::build_args_from_file(path).context(ParseBuildArgumentsSnafu)?;
            user_container_build_args.extend(build_arguments_from_file);
        }

        let target = BakefileTarget::common(
            date_time,
            revision,
            cli_args.vendor_version.clone(),
            container_build_args,
            user_container_build_args,
            metadata,
        );

        Ok(target)
    }

    fn from_targets(
        targets: Targets,
        cli_args: &cli::BuildArguments,
        config: Config,
    ) -> Result<Self, Error> {
        let floating_vendor_version =
            utils::parse_floating_vendor_version(&cli_args.vendor_version, cli_args.floating_tag)
                .context(ParseFloatingVendorVersionSnafu)?;

        let mut bakefile_targets = BTreeMap::new();
        let mut groups: BTreeMap<String, BakefileGroup> = BTreeMap::new();

        // Destructure config so that we can move and borrow fields separately.
        let Config {
            build_arguments,
            metadata,
            ..
        } = config;

        // Create a common target, which contains shared data, like annotations, arguments, labels, etc...
        let common_target = Self::common_target(cli_args, build_arguments, &metadata)?;
        bakefile_targets.insert(COMMON_TARGET_NAME.to_owned(), common_target);

        // The image registry, eg. `oci.stackable.tech` or `localhost`
        let image_registry = if cli_args.use_localhost_registry {
            &HostPort::localhost()
        } else {
            &cli_args.registry
        };

        for (image_name, (image_config, is_entry)) in targets.into_iter() {
            for (image_version, image_options) in image_config.versions {
                let image_repository_uri = utils::format_image_repository_uri(
                    image_registry,
                    &cli_args.registry_namespace,
                    &image_name,
                );

                let image_index_manifest_tag = utils::format_image_index_manifest_tag(
                    &image_version,
                    &metadata.vendor_tag_prefix,
                    &cli_args.vendor_version,
                );

                let image_manifest_tag = utils::format_image_manifest_tag(
                    &image_index_manifest_tag,
                    cli_args.target_platform.architecture(),
                    cli_args.strip_architecture,
                );

                let image_manifest_uri =
                    utils::format_image_manifest_uri(&image_repository_uri, &image_manifest_tag);

                // TODO (@Techassi): Clean this up
                // TODO (@Techassi): Move the arg formatting into functions
                // Start off with the shared (across all versions of the same image) build arguments.
                let mut build_arguments = image_config.build_arguments.clone();

                let local_version_docker_args: docker::BuildArguments = image_options
                    .local_images
                    .iter()
                    .map(|(image_name, image_version)| {
                        let key = docker::BuildArgumentKey::local_image_key(image_name);
                        (key, image_version.to_owned())
                    })
                    .collect();

                build_arguments.extend(image_options.build_arguments);
                build_arguments.extend(local_version_docker_args);

                // TODO (@Techassi): Rename this to IMAGE_VERSION
                build_arguments.insert("PRODUCT_VERSION".into(), image_version.to_string());
                build_arguments.insert("IMAGE_REPOSITORY_URI".into(), image_repository_uri.clone());
                build_arguments.insert("IMAGE_INDEX_MANIFEST_TAG".into(), image_index_manifest_tag);
                build_arguments.insert("IMAGE_MANIFEST_TAG".into(), image_manifest_tag);
                build_arguments.insert("IMAGE_MANIFEST_URI".into(), image_manifest_uri.clone());

                let tags = if let Some(floating_vendor_version) = floating_vendor_version.as_deref()
                {
                    let image_index_manifest_floating_tag = utils::format_image_index_manifest_tag(
                        &image_version,
                        &metadata.vendor_tag_prefix,
                        floating_vendor_version,
                    );

                    let image_manifest_floating_tag = utils::format_image_manifest_tag(
                        &image_index_manifest_floating_tag,
                        cli_args.target_platform.architecture(),
                        cli_args.strip_architecture,
                    );

                    let floating_image_manifest_uri = utils::format_image_manifest_uri(
                        &image_repository_uri,
                        &image_manifest_floating_tag,
                    );

                    TagSet::with_others(image_manifest_uri, vec![floating_image_manifest_uri])
                } else {
                    TagSet::new(image_manifest_uri)
                };

                // By using a cap-std Dir, we can ensure that the paths provided must be relative to
                // the appropriate image folder and wont escape it by providing absolute or relative
                // paths with traversals (..).
                let image_dir = Dir::open_ambient_dir(&image_name, ambient_authority())
                    .with_context(|_| OpenScopedDirectorySnafu {
                        path: image_name.clone(),
                    })?;

                let dockerfile_path = if let Some(custom_path) = &image_options.dockerfile {
                    ensure!(
                        image_dir.exists(custom_path),
                        NoSuchContainerfileExistsSnafu { path: image_name }
                    );

                    PathBuf::new().join(&image_name).join(custom_path)
                } else {
                    ensure!(
                        image_dir.exists(&cli_args.target_containerfile),
                        NoSuchContainerfileExistsSnafu { path: image_name }
                    );

                    PathBuf::new()
                        .join(&image_name)
                        .join(&cli_args.target_containerfile)
                };

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

                let annotations = BakefileTarget::image_version_annotation(
                    &image_version,
                    &metadata.vendor_tag_prefix,
                    &cli_args.vendor_version,
                );

                let target = BakefileTarget {
                    image_name: image_name.clone(),
                    tags,
                    arguments: build_arguments,
                    platforms: vec![cli_args.target_platform.clone()],
                    // NOTE (@Techassi): Should this instead be scoped to the folder of the image we build
                    context: Some(PathBuf::from(".")),
                    dockerfile: Some(dockerfile_path),
                    inherits: vec![COMMON_TARGET_NAME.to_owned()],
                    annotations,
                    contexts,
                    ..Default::default()
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
        // Replace any slashes from nested image names, eg. shared/protobuf, because docker buildx
        // has this weird restriction (because it also supports push, which we do on our own). We
        // are therefore artificially limited what target names we can use: [a-zA-Z0-9_-]+
        let image_name = image_name.replace('/', "__");

        // The dots in the semantic version also need to be replaced.
        let image_version = image_version.replace('.', "_");

        format!("{image_name}-{image_version}")
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
#[derive(Debug, Default, Serialize)]
pub struct BakefileTarget {
    /// Only used internally to be able to access the original image name of the target. This field
    /// is not serialized into the final Bakefile.
    #[serde(skip)]
    pub image_name: String,

    /// Defines build arguments for the target.
    #[serde(
        rename = "args",
        skip_serializing_if = "docker::BuildArguments::is_empty"
    )]
    pub arguments: docker::BuildArguments,

    /// Adds annotations to images built with bake.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<String>,

    /// Specifies the location of the build context to use for this target.
    ///
    /// Accepts a URL or a directory path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<PathBuf>,

    /// Additional build contexts.
    ///
    /// This attribute takes a map, where keys result in named contexts that you can reference in
    /// your builds.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub contexts: BTreeMap<String, String>,

    /// Name of the Dockerfile to use for the build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dockerfile: Option<PathBuf>,

    /// A target can inherit attributes from other targets.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inherits: Vec<String>,

    /// Assigns image labels to the build.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,

    // TODO (@Techassi): Explore how we can build multiple platforms at once
    /// Set target platforms for the build target.
    ///
    /// Technically, multiple architectures can be listed in here, but boil chooses to build only
    /// one architecture at a time.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub platforms: Vec<TargetPlatform>,

    /// Image names and tags to use for the build target.
    #[serde(
        serialize_with = "TagSet::serialize_for_bakefile",
        // DO NOT REMOVE: This is needed for correct serialization of the common target.
        skip_serializing_if = "TagSet::serde_is_empty"
    )]
    pub tags: TagSet,
}

impl BakefileTarget {
    fn common(
        date_time: String,
        revision: String,
        release_version: String,
        container_build_args: docker::BuildArguments,
        user_container_build_args: docker::BuildArguments,
        metadata: &MetadataOptions,
    ) -> Self {
        let config::MetadataOptions {
            documentation: docs,
            licenses,
            authors,
            source,
            vendor,
            ..
        } = metadata;

        // Annotations describe OCI image components.
        // Add annotations which are always present.
        let mut annotations = vec![
            format!("{ANNOTATION_CREATED}={date_time}"),
            format!("{ANNOTATION_REVISION}={revision}"),
        ];

        // Add optional annotations.
        if let Some(authors) = authors {
            annotations.push(format!("{ANNOTATION_AUTHORS}={authors}"));
        }

        if let Some(docs) = docs {
            annotations.push(format!("{ANNOTATION_DOCUMENTATION}={docs}"));
        }

        if let Some(source) = source {
            annotations.push(format!("{ANNOTATION_SOURCE}={source}"));
        }

        if let Some(licenses) = licenses {
            annotations.push(format!("{ANNOTATION_LICENSES}={licenses}"));
        }

        if let Some(vendor) = vendor {
            annotations.push(format!("{ANNOTATION_VENDOR}={vendor}"));
        }

        let mut arguments = container_build_args;
        arguments.extend(user_container_build_args);
        arguments.insert("RELEASE_VERSION".into(), release_version);

        // Labels describe Docker resources, and con be considered legacy. We
        // should use annotations instead. These labels are only added to be
        // consistent with `bake`.
        let labels = BTreeMap::from([
            (ANNOTATION_CREATED.to_owned(), date_time.clone()),
            (ANNOTATION_REVISION.to_owned(), revision),
            (DOCKER_LABEL_BUILD_DATE.to_owned(), date_time),
        ]);

        Self {
            annotations,
            arguments,
            labels,
            ..Default::default()
        }
    }

    fn image_version_annotation(
        image_version: &str,
        vendor_tag_prefix: &str,
        vendor_image_version: &str,
    ) -> Vec<String> {
        let image_index_manifest_tag = utils::format_image_index_manifest_tag(
            image_version,
            vendor_tag_prefix,
            vendor_image_version,
        );

        vec![format!("{ANNOTATION_VERSION}={image_index_manifest_tag}")]
    }
}

#[derive(Debug, Default, Serialize)]
pub struct BakefileGroup {
    targets: Vec<String>,
}

/// A set of tags which contains a single canonical tag and N additional, other tags.
//
// The derived serialization is used for structured output via the `--write-image-manifest-uris`
// CLI argument.
#[derive(Debug, Serialize)]
pub struct TagSet {
    /// A single canonical tag.
    canonical: String,
    others: Vec<String>,

    /// Marks this tag set as empty..
    ///
    /// To be able to use [`BakefileTarget`] for the common (inherited) target, we want to leverage
    /// its own and its subtypes [`Default`] implementation. That's because the common target only
    /// sets a few selected fields and uses [`Default`] for all the rest. These fields all get
    /// skipped during serialization because they are all empty/unset.
    ///
    /// A [`TagSet`] by definition must contain a canonical value for "normal" targets. However, for
    /// the common target, there is no canonical tag, and we additionally don't want to serialize
    /// the field.
    ///
    /// As such, [`TagSet`] offers [`TagSet::empty`] in this module only to construct an empty
    /// [`TagSet`] to be used in the common target. It won't get serialized as serde skips it if
    /// [`TagSet::is_empty`] returns true. This function returns the value of this field.
    //
    // NOTE (@Techassi): I know this is a kind of dirty "hack" to make the code work how it should,
    // but I wasn't able to come up with a better/more elegant solution without other pain points.
    // I tried using an enum with two variants, but the matching got annoying and there just aren't
    // sound implementations of associated functions in some cases. One other option is to somehow
    // make the `tags` field of the BakefileTarget generic and to accept two different
    // implementations: A real one, and a noop one. But this most likely comes with all the baggage
    // we know and love (or hate) of generics.
    #[serde(skip)]
    is_empty: bool,
}

impl Default for TagSet {
    fn default() -> Self {
        Self::empty()
    }
}

impl Display for TagSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.others.is_empty() {
            write!(f, "{canonical}", canonical = self.canonical)
        } else {
            write!(
                f,
                "{canonical} ({others})",
                canonical = self.canonical,
                others = self.others.join(",")
            )
        }
    }
}

impl TagSet {
    /// Creates and returns a new [`TagSet`] which only contains the canonical tag.
    pub fn new(canonical: String) -> Self {
        Self {
            canonical,
            others: Vec::new(),
            is_empty: false,
        }
    }

    /// Creates and returns a new [`TagSet`] which both contains the canonical and other tags.
    pub fn with_others(canonical: String, others: Vec<String>) -> Self {
        let mut tag_set = Self::new(canonical);
        tag_set.others = others;
        tag_set
    }

    /// Create an empty [`TagSet`].
    ///
    /// See the `is_empty` field for more details on this.
    fn empty() -> Self {
        Self {
            canonical: String::new(),
            others: Vec::new(),
            is_empty: true,
        }
    }

    /// Returns if the [`TagSet`] is empty.
    ///
    /// Used for skipping serialization in the common target of the [`Bakefile`].
    fn serde_is_empty(&self) -> bool {
        self.is_empty
    }

    /// Special serialization implementation when serialized as part of a [`Bakefile`].
    fn serialize_for_bakefile<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // The number of other tags + 1 for the canonical tag.
        let len = Some(self.others.len() + 1);
        let mut seq = serializer.serialize_seq(len)?;

        seq.serialize_element(&self.canonical)?;

        for other in &self.others {
            seq.serialize_element(other)?;
        }

        seq.end()
    }
}
