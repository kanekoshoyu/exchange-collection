use clap::Parser;
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

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
