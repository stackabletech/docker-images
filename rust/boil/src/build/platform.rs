use std::{fmt::Display, str::FromStr};

use serde::Serialize;
use snafu::{OptionExt as _, ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum ParseArchitecturePairError {
    #[snafu(display("encountered invalid format, expected platform/architecture"))]
    InvalidFormat,

    #[snafu(display("failed to parse architecture"))]
    ParseArchitecture { source: strum::ParseError },

    #[snafu(display("unsupported, target platform"))]
    UnsupportedPlatform,
}

#[derive(Clone, Debug)]
pub enum TargetPlatform {
    Linux(Architecture),
}

impl Serialize for TargetPlatform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for TargetPlatform {
    type Err = ParseArchitecturePairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (platform, architecture) = s.split_once('/').context(InvalidFormatSnafu)?;
        let architecture = Architecture::from_str(architecture).context(ParseArchitectureSnafu)?;

        match platform {
            "linux" => Ok(Self::Linux(architecture)),
            _ => UnsupportedPlatformSnafu.fail(),
        }
    }
}

impl Display for TargetPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TargetPlatform::Linux(architecture) => write!(f, "linux/{architecture}"),
        }
    }
}

impl TargetPlatform {
    pub fn architecture(&self) -> &Architecture {
        match self {
            TargetPlatform::Linux(architecture) => architecture,
        }
    }
}

#[derive(Copy, Clone, Debug, strum::Display, strum::EnumString, strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Architecture {
    Amd64,
    Arm64,
}
