use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::{Deserialize, Serialize, de::Visitor};
use snafu::{OptionExt, ResultExt, Snafu, ensure};

#[derive(Debug, Snafu)]
pub enum ParseBuildArgumentError {
    #[snafu(display("invalid format, expected <key>=<value>"))]
    InvalidFormat,

    #[snafu(display("encountered non ASCII characters"))]
    NonAscii,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct BuildArgumentKey(String);

impl<T> From<T> for BuildArgumentKey
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug)]
pub struct BuildArgument {
    pub key: BuildArgumentKey,
    pub value: String,
}

impl FromStr for BuildArgument {
    type Err = ParseBuildArgumentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(s.is_ascii(), NonAsciiSnafu);

        let (key, value) = s.split_once('=').context(InvalidFormatSnafu)?;
        let key = BuildArgumentKey::new(key);

        Ok(Self {
            key,
            value: value.to_owned(),
        })
    }
}

impl BuildArgumentKey {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self(name.as_ref().replace(['-', '/'], "_").to_uppercase())
    }

    pub fn local_image_key(image_name: &str) -> Self {
        Self::new(format!("{image_name}_VERSION"))
    }
}

#[derive(Debug, Snafu)]
pub enum ParseBuildArgumentsError {
    #[snafu(display("failed to read file at {path}", path = path.display()))]
    ReadFile {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to parse build argument"))]
    ParseBuildArgument { source: ParseBuildArgumentError },
}

pub type BuildArguments = BTreeMap<BuildArgumentKey, String>;

/// Custom [`serde::Deserialize`] implementation to ensure we properly format the keys.
pub fn deserialize_args<'de, D>(deserializer: D) -> Result<BuildArguments, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct BuildArgumentsVisitor;

    impl<'de> Visitor<'de> for BuildArgumentsVisitor {
        type Value = BuildArguments;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a map of valid build arguments")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut args = BTreeMap::new();

            while let Some((key, value)) = map.next_entry::<&str, _>()? {
                args.insert(BuildArgumentKey::new(key), value);
            }

            Ok(args)
        }
    }

    deserializer.deserialize_map(BuildArgumentsVisitor)
}

// We sadly cannot use a From<Vec<BuildArgument>> for BuildArguments impl here because of the
// orphan rule.
pub fn build_args_vec_to_btree_map(vec: Vec<BuildArgument>) -> BuildArguments {
    vec.into_iter().map(|arg| (arg.key, arg.value)).collect()
}

pub fn build_args_from_file<P>(path: P) -> Result<BuildArguments, ParseBuildArgumentsError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let content = std::fs::read_to_string(path).context(ReadFileSnafu { path })?;
    let mut args = BTreeMap::new();

    for line in content.lines() {
        let arg = BuildArgument::from_str(line).context(ParseBuildArgumentSnafu)?;
        args.insert(arg.key, arg.value);
    }

    Ok(args)
}
