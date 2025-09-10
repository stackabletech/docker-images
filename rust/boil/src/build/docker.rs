use std::{
    collections::BTreeSet,
    fmt::Display,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::{Deserialize, Serialize, de::Visitor, ser::SerializeMap};
use snafu::{OptionExt, ResultExt, Snafu, ensure};

#[derive(Debug, Snafu)]
pub enum ParseBuildArgumentError {
    #[snafu(display("invalid format, expected <key>=<value>"))]
    InvalidFormat,

    #[snafu(display("encountered non ASCII characters"))]
    NonAscii,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BuildArgument((String, String));

impl BuildArgument {
    pub fn new(key: String, value: String) -> Self {
        let key = Self::format_key(key);
        Self((key, value))
    }

    fn format_key(key: impl AsRef<str>) -> String {
        key.as_ref().replace(['-', '/'], "_").to_uppercase()
    }
}

impl FromStr for BuildArgument {
    type Err = ParseBuildArgumentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(s.is_ascii(), NonAsciiSnafu);

        let (key, value) = s.split_once('=').context(InvalidFormatSnafu)?;
        let key = Self::format_key(key);

        Ok(Self((key, value.to_owned())))
    }
}

impl<'de> Deserialize<'de> for BuildArgument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BuildArgumentVisitor;

        impl Visitor<'_> for BuildArgumentVisitor {
            type Value = BuildArgument;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a valid build argument")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                BuildArgument::from_str(v).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_str(BuildArgumentVisitor)
    }
}

impl Display for BuildArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (key, value) = &self.0;
        write!(f, "{key}={value}")
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

#[derive(Clone, Debug, Default)]
pub struct BuildArguments(BTreeSet<BuildArgument>);

impl Deref for BuildArguments {
    type Target = BTreeSet<BuildArgument>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BuildArguments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Extend<BuildArgument> for BuildArguments {
    fn extend<T: IntoIterator<Item = BuildArgument>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl IntoIterator for BuildArguments {
    type Item = BuildArgument;

    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'de> Deserialize<'de> for BuildArguments {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
                let mut args = BTreeSet::new();

                while let Some((key, value)) = map.next_entry()? {
                    args.insert(BuildArgument::new(key, value));
                }

                Ok(BuildArguments(args))
            }
        }

        deserializer.deserialize_map(BuildArgumentsVisitor)
    }
}

impl Serialize for BuildArguments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;

        for BuildArgument((key, value)) in &self.0 {
            map.serialize_entry(&key, &value)?;
        }

        map.end()
    }
}

impl BuildArguments {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn from_file<P>(path: P) -> Result<Self, ParseBuildArgumentsError>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).context(ReadFileSnafu { path })?;
        let mut args = Self::new();

        for line in content.lines() {
            let arg = BuildArgument::from_str(line).context(ParseBuildArgumentSnafu)?;
            args.insert(arg);
        }

        Ok(args)
    }
}
