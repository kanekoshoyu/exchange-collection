use clap::Parser;
use eyre::{eyre, Result};
use serde::{Deserialize, Deserializer, Serialize};
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
#[derive(Deserialize, Debug, Clone)]
pub struct AsyncApiInfo {
    #[serde(rename = "asyncapi")]
    pub version: Version,
}
#[derive(Deserialize, Debug, Clone)]
pub struct OpenApiInfo {
    #[serde(rename = "openapi")]
    pub version: Version,
}

#[derive(Parser, Debug)]
pub struct CliInput {
    #[arg(long)]
    /// Some when batch
    pub input_directory: Option<PathBuf>,
    /// None when batch, Some when we convert one file only
    #[arg(long)]
    pub input_filename: Option<PathBuf>,
    // None when batch, Some when select one
    #[arg(long)]
    pub output_directory: Option<PathBuf>,
    // Empty when all all languages are in target
    #[arg(long)]
    pub output_language: Vec<ProgrammingLanguage>,
}
impl Default for CliInput {
    fn default() -> Self {
        Self {
            input_directory: Some(PathBuf::from_str("asset").unwrap()),
            input_filename: None,
            output_directory: Some(PathBuf::from_str("target").unwrap()),
            output_language: ProgrammingLanguage::iter().collect(),
        }
    }
}
impl CliInput {
    /// gives the legal input
    pub fn load() -> Result<Self> {
        let default = CliInput::default();
        let mut input: CliInput = CliInput::parse();
        if input.output_directory.is_none() {
            input.output_directory = default.output_directory;
        }
        if input.output_language.is_empty() {
            input.output_language = default.output_language;
        }
        match (&input.input_filename, &input.input_directory) {
            (None, None) => {
                input.input_directory = default.input_directory;
            }
            (Some(_), Some(_)) => {
                return Err(eyre!(
                    "both input_filename and input_directory are provided, please pick one"
                ))
            }
            _ => {}
        }

        Ok(input)
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Deserialize,
    Serialize,
    EnumIter,
    EnumString,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
)]
pub enum ProgrammingLanguage {
    #[strum(serialize = "rust")]
    Rust,
    #[strum(serialize = "python")]
    Python,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Deserialize,
    Serialize,
    EnumIter,
    EnumString,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
)]
pub enum ApiFileFormat {
    #[strum(serialize = "openapi")]
    OpenApi,
    #[strum(serialize = "asyncapi")]
    AsyncApi,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Deserialize,
    Serialize,
    EnumIter,
    EnumString,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
)]
pub enum Protocol {
    #[strum(serialize = "rest")]
    Rest,
    #[strum(serialize = "ws")]
    Ws,
    #[strum(serialize = "fix")]
    Fix,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
}
impl Add for Version {
    type Output = Version;

    fn add(self, rhs: Self) -> Self::Output {
        Version {
            major: self.major + rhs.major,
            minor: self.minor + rhs.minor,
            patch: self.patch + rhs.patch,
        }
    }
}
impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Define a custom visitor to handle deserialization
        struct VersionVisitor;

        impl<'de> serde::de::Visitor<'de> for VersionVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a version string in the format 'major.minor.patch'")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Split the version string by dots
                let parts: Vec<&str> = v.split('.').collect();
                if parts.len() != 3 {
                    return Err(serde::de::Error::custom("invalid version string format"));
                }

                // Parse each part into an integer
                let major = parts[0]
                    .parse::<usize>()
                    .map_err(serde::de::Error::custom)?;
                let minor = parts[1]
                    .parse::<usize>()
                    .map_err(serde::de::Error::custom)?;
                let patch = parts[2]
                    .parse::<usize>()
                    .map_err(serde::de::Error::custom)?;

                Ok(Version {
                    major,
                    minor,
                    patch,
                })
            }
        }

        // Use the custom visitor to deserialize the string
        deserializer.deserialize_str(VersionVisitor)
    }
}
pub struct InputFileParameter {
    /// we keep adding exchanges, no pub enum
    pub exchange: String,
    pub protocol: Protocol,
    pub format: ApiFileFormat,
    pub version: Version,
}
impl InputFileParameter {
    pub fn from_filename(filename: impl AsRef<Path>) -> Result<Self, ()> {
        let filename = filename.as_ref();
        if !filename.is_file() {
            println!("file does not exist");
            return Err(());
        }

        // get content first
        let file_content = std::fs::read_to_string(filename).expect("Failed to read YAML file");

        // "binance_ws_asyncapi.yaml"
        let filename = filename.file_name().unwrap();
        let filename = filename.to_str().unwrap();

        // "binance_ws_asyncapi"
        if !filename.contains(".yaml") {
            return Err(());
        }
        let rest = filename.to_string().replace(".yaml", "");

        // "binance", "ws", "asyncapi"
        let str_vec: Vec<&str> = rest.split("_").collect();

        if str_vec.len() != 3 {
            println!("invalid format");
            return Err(());
        }
        let exchange = str_vec[0].to_string();
        let protocol = Protocol::from_str(str_vec[1]).map_err(|_| ())?;
        let format = ApiFileFormat::from_str(str_vec[2]).map_err(|_| ())?;

        let version = match format {
            ApiFileFormat::OpenApi => {
                let info: OpenApiInfo =
                    serde_yaml::from_str(&file_content).expect("Failed to parse YAML");
                info.version
            }
            ApiFileFormat::AsyncApi => {
                let info: AsyncApiInfo =
                    serde_yaml::from_str(&file_content).expect("Failed to parse YAML");
                info.version
            }
        };

        Ok(InputFileParameter {
            exchange,
            protocol,
            format,
            version,
        })
    }
}
