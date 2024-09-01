use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Parser, Debug)]
struct CliInput {
    #[arg(long)]
    /// Some when batch
    input_directory: Option<PathBuf>,
    /// None when batch, Some when we convert one file only
    #[arg(long)]
    input_filename: Option<PathBuf>,
    // None when batch, Some when select one
    #[arg(long)]
    output_directory: Option<PathBuf>,
    // Empty when all all languages are in target
    #[arg(long)]
    output_language: Vec<ProgrammingLanguage>,
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
enum ProgrammingLanguage {
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
enum ApiFileFormat {
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
enum Protocol {
    #[strum(serialize = "rest")]
    Rest,
    #[strum(serialize = "ws")]
    Ws,
    #[strum(serialize = "fix")]
    Fix,
}

struct InputFileParameter {
    /// we keep adding exchanges, no enum
    exchange: String,
    protocol: Protocol,
    format: ApiFileFormat,
}
impl InputFileParameter {
    pub fn from_filename(filename: impl AsRef<Path>) -> Result<Self, ()> {
        let filename = filename.as_ref();
        if !filename.is_file() {
            println!("file does not exist");
            return Err(());
        }

        // "binance_ws_asyncapi.yaml"
        let filename = filename.file_name().unwrap();
        let filename = filename.to_str().unwrap();

        // "binance_ws_asyncapi"
        if !filename.contains(".yaml") {
            return Err(());
        }
        let rest = format!("{}", filename).replace(".yaml", "");

        // "binance", "ws", "asyncapi"
        let str_vec: Vec<&str> = rest.split("_").collect();

        if str_vec.len() != 3 {
            println!("invalid format");
            return Err(());
        }

        // Ok(InputFileParameter {
        //     exchange: str_vec[0].to_string(),
        //     protocol: str_vec[1].parse::<Protocol>().map_err(|_e| ())?,
        //     format: str_vec[2].parse::<ApiFileFormat>().map_err(|_e| ())?,
        // })
        Ok(InputFileParameter {
            exchange: str_vec[0].to_string(),
            protocol: Protocol::from_str(str_vec[1]).map_err(|_| ())?,
            format: ApiFileFormat::from_str(str_vec[2]).map_err(|_| ())?,
        })
    }
}

fn run() -> Result<(), ()> {
    // load
    let cli: CliInput = CliInput::load()?;
    println!("{:#?}", cli);

    match (cli.input_filename, cli.input_directory) {
        (None, Some(input_dir)) => {
            // batch load from input_dir
            println!("batch load");

            let files = std::fs::read_dir(input_dir.clone()).unwrap();
            // gather all filenames
            let mut filenames = Vec::new();
            for file in files {
                let file = file.map_err(|_| ())?;
                let mut filename = input_dir.clone();
                filename.push(file.file_name());
                filenames.push(filename);
            }

            let output_directory = cli.output_directory.unwrap();
            for input_filename in filenames {
                for output_language in cli.output_language.clone() {
                    let mut command = codegen_command(
                        input_filename.clone(),
                        output_directory.clone(),
                        output_language,
                    )?;
                    println!("{:?}", command);
                    match command.output() {
                        Ok(o) => println!("codegen success\n{o:?}"),
                        Err(e) => println!("codegen fail, {e}"),
                    }
                }
            }
        }
        (Some(input_filename), None) => {
            // single load
            println!("single load");
            let output_directory = cli.output_directory.unwrap();
            for output_language in cli.output_language.clone() {
                let command = codegen_command(
                    input_filename.clone(),
                    output_directory.clone(),
                    output_language,
                );
                println!("{:?}", command);
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

/// openapi-generator-cli generate -i example_openapi.yaml -g <language> -o output/example_rust_model
/// asyncapi generate models <language> example_asyncapi.yml -o output/example_<language>>_model
fn codegen_command(
    input_filename: impl AsRef<Path>,
    output_directory: impl AsRef<Path>,
    language: ProgrammingLanguage,
) -> Result<Command, ()> {
    let param = InputFileParameter::from_filename(&input_filename)?;
    let exchange = param.exchange;
    let protocol = param.protocol;
    let format = param.format;

    // define subpath under output_directory to output
    let sub_path_str = match language {
        ProgrammingLanguage::Rust => format!("{}/{}/src/{}", language, exchange, protocol),
        ProgrammingLanguage::Python => format!("{}/{}/{}", language, exchange, protocol),
    };
    let sub_path = PathBuf::from_str(&sub_path_str).map_err(|_| ())?;
    // append subpath into the output_directory
    let mut output_directory = output_directory.as_ref().to_path_buf();
    output_directory.push(sub_path);

    let output_directory = Path::new(&output_directory);
    // let output_directory = output_directory.canonicalize().unwrap();

    // output
    let input_filename = input_filename.as_ref();
    Ok(match format {
        ApiFileFormat::OpenApi => {
            let mut cmd = Command::new("openapi-generator-cli");
            cmd.arg("generate");
            cmd.arg(format!("-g {}", language));
            cmd.arg(format!("-i {}", input_filename.display()));
            cmd.arg(format!("-o {}", output_directory.display()));
            cmd
        }
        ApiFileFormat::AsyncApi => {
            let mut cmd = Command::new("asyncapi");
            cmd.arg("generate models");
            cmd.arg(format!("{}", language));
            cmd.arg(format!("-i {}", input_filename.display()));
            cmd.arg(format!("-o {}", output_directory.display()));
            cmd
        }
    })
}

pub fn main() {
    match run() {
        Ok(_) => println!("success"),
        Err(e) => println!("error detected, {e:?}"),
    }
}

#[cfg(test)]
mod tests {
    use std::{ffi::OsStr, path::PathBuf, str::FromStr};

    #[test]
    fn test_codegen_command() {
        use super::*;
        let input_filename = PathBuf::from_str("../asset/binance_ws_asyncapi.yaml").unwrap();
        let output_directory = PathBuf::from_str("target").unwrap();
        let output_language = ProgrammingLanguage::Rust;
        let command = match codegen_command(input_filename, output_directory, output_language) {
            Ok(command) => command,
            Err(e) => panic!("{e:?}"),
        };
        // TODO come up with a way to compare the command
        let args: Vec<&OsStr> = command.get_args().collect();
        let program = command.get_program();
        assert_eq!(program, "asyncapi");
        assert_eq!(
            args,
            [
                "generate models",
                "rust",
                "-i ../asset/binance_ws_asyncapi.yaml",
                "-o target/rust/binance/src/ws"
            ]
            .to_vec()
        );
    }

    #[test]
    fn test_deserialize_language() {
        use super::*;
        match ProgrammingLanguage::from_str("python") {
            Ok(language) => assert_eq!(language, ProgrammingLanguage::Python),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_serialize_language() {
        use super::*;
        let language = ProgrammingLanguage::Python;
        assert_eq!(format!("{language}"), "python");
    }

    #[test]
    fn test_parse_input_file() {
        use super::*;
        match InputFileParameter::from_filename("../asset/binance_ws_asyncapi.yaml") {
            Ok(parameter) => {
                assert_eq!(parameter.exchange, "binance");
                assert_eq!(parameter.protocol, Protocol::Ws);
                assert_eq!(parameter.format, ApiFileFormat::AsyncApi);
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn test_path() {
        let str_path = "../asset/binance_ws_asyncapi.yaml";
        let path = PathBuf::from_str(str_path).unwrap();
        let canonical = path.canonicalize().unwrap();
        assert!(canonical.is_file());
    }

    #[test]
    fn test_format() {
        let str_path = "../asset/binance_ws_asyncapi.yaml";
        assert!(str_path.ends_with(".yaml"));
        // pathbuf we cannot use ends_with
        // let str_path = PathBuf::from(str_path);
        // assert!(!str_path.ends_with(".yaml"));
    }
}
