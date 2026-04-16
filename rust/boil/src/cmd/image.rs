use std::{collections::BTreeMap, io::IsTerminal};

use secrecy::{ExposeSecret, SecretString};
use snafu::{ResultExt, Snafu, ensure};

use crate::{
    cli::{ImageCheckArguments, ImageListArguments, Pretty},
    config::Config,
    core::bakefile::{self, Targets, TargetsOptions},
    models::TagList,
    utils::format_image_index_manifest_tag,
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to serialize list as JSON"))]
    SerializeList {
        source: serde_json::Error,
    },

    #[snafu(display("failed to build list of targets"))]
    BuildTargets {
        source: bakefile::TargetsError,
    },

    #[snafu(display("failed to build request client"))]
    BuildClient {
        source: reqwest::Error,
    },

    #[snafu(display("failed to send request"))]
    SendRequest {
        source: reqwest::Error,
    },

    #[snafu(display("failed to deserialize response"))]
    DeserializeResponse {
        source: reqwest::Error,
    },

    MissingVersion {
        image_name: String,
    },
}

/// This is the `boil show images` command handler function.
pub fn list_images(arguments: ImageListArguments) -> Result<(), Error> {
    let targets = if arguments.image.is_empty() {
        Targets::all(TargetsOptions {
            only_entry: true,
            flat: false,
        })
        .context(BuildTargetsSnafu)?
    } else {
        Targets::set(
            &arguments.image,
            TargetsOptions {
                only_entry: true,
                flat: false,
            },
        )
        .context(BuildTargetsSnafu)?
    };

    let list = targets
        .into_iter()
        .map(|(image_name, (image_config, _))| {
            let versions: Vec<_> = image_config
                .versions
                .into_iter()
                .map(|(image_version, _)| image_version)
                .collect();
            (image_name, versions)
        })
        .collect();

    print_to_stdout(list, arguments.pretty)
}

pub async fn check_images(arguments: ImageCheckArguments, config: Config) -> Result<(), Error> {
    let targets = if arguments.image.is_empty() {
        Targets::all(TargetsOptions {
            only_entry: true,
            flat: true,
        })
        .context(BuildTargetsSnafu)?
    } else {
        Targets::set(
            &arguments.image,
            TargetsOptions {
                only_entry: true,
                flat: false,
            },
        )
        .context(BuildTargetsSnafu)?
    };

    let registry_token = std::env::var("REGISTRY_TOKEN").ok().map(SecretString::from);
    let client = reqwest::ClientBuilder::new()
        .build()
        .context(BuildClientSnafu)?;

    for (image_name, (image_config, _)) in targets {
        // TODO (@Techassi): Do these checks for all registries
        let Some((registry, registry_options)) = image_config.metadata.registries.first_key_value()
        else {
            continue;
        };

        println!(
            "Checking for {registry}/{registry_namespace}/{image_name}",
            registry_namespace = registry_options.namespace,
        );

        let url = format!(
            "https://{registry}/v2/{registry_namespace}/{image_name}/tags/list",
            registry_namespace = registry_options.namespace,
        );
        let request = client.get(url);

        let request = match &registry_token {
            Some(registry_token) => request.bearer_auth(registry_token.expose_secret()),
            None => request,
        };

        let tag_list: TagList = request
            .send()
            .await
            .context(SendRequestSnafu)?
            .json()
            .await
            .context(DeserializeResponseSnafu)?;

        ensure!(
            image_config.versions.iter().all(|(image_version, _)| {
                let index_manifest_tag = format_image_index_manifest_tag(
                    image_version,
                    &config.metadata.vendor_tag_prefix,
                    &arguments.image_version,
                );

                println!("- {image_name}:{index_manifest_tag}");
                tag_list.tags.contains(&index_manifest_tag)
            }),
            MissingVersionSnafu { image_name }
        );
    }

    Ok(())
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
