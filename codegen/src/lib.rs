use clap::Parser;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Display;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Deserialize, Debug, Clone)]
pub struct AsyncApiInfo {
    #[serde(rename = "asyncapi")]
    version: Version,
}
#[derive(Deserialize, Debug, Clone)]
pub struct OpenApiInfo {
    #[serde(rename = "openapi")]
    version: Version,
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
    pub fn load() -> Result<Self, ()> {
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
            (Some(_), Some(_)) => return Err(()),
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

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Version {
    major: usize,
    minor: usize,
    patch: usize,
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
impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        pub struct VersionVisitor;

        impl<'de> serde::de::Visitor<'de> for VersionVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a Rust type, like Vec<i64>")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Version, E> {
                let values: Vec<&str> = value.split(".").collect();
                let error = serde::de::Error::custom("failed parsing");
                if values.len() != 3 {
                    return Err(error);
                }
                Ok(Version {
                    major: values[0]
                        .parse::<usize>()
                        .map_err(|_| serde::de::Error::custom("failed parsing"))?,
                    minor: values[0]
                        .parse::<usize>()
                        .map_err(|_| serde::de::Error::custom("failed parsing"))?,
                    patch: values[0]
                        .parse::<usize>()
                        .map_err(|_| serde::de::Error::custom("failed parsing"))?,
                })
            }
        }

        deserializer.deserialize_str(VersionVisitor)
    }
}
impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
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
