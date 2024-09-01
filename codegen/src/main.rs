use clap::Parser;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

#[derive(Parser, Debug)]
struct CliInput {
    #[arg(short, long)]
    /// Some when batch
    input_directory: Option<String>,
    /// None when batch, Some when we convert one file only
    input_filename: Option<String>,
    output_directory: Option<String>,
    // None when batch, Some when select one
    output_language: Vec<ProgrammingLanguage>,
}
impl Default for CliInput {
    fn default() -> Self {
        Self {
            input_directory: Some("asset".to_string()),
            input_filename: None,
            output_directory: Some("target".to_string()),
            output_language: ProgrammingLanguage::iter().collect(),
        }
    }
}

impl CliInput {
    /// gives the legal input
    pub fn load() -> Result<Self, ()> {
        let default = CliInput::default();
        let mut input: CliInput = CliInput::parse();
        // default set to all
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
    Clone, Copy, Debug, Deserialize, Serialize, EnumIter, EnumString, PartialEq, PartialOrd, Eq, Ord,
)]
#[serde(rename_all = "lowercase")]
enum ProgrammingLanguage {
    Rust,
    Python,
}

#[derive(
    Clone, Copy, Debug, Deserialize, Serialize, EnumIter, EnumString, PartialEq, PartialOrd, Eq, Ord,
)]
#[serde(rename_all = "lowercase")]
enum ApiFileFormat {
    OpenApi,
    AsyncApi,
}

#[derive(
    Clone, Copy, Debug, Deserialize, Serialize, EnumIter, EnumString, PartialEq, PartialOrd, Eq, Ord,
)]
#[serde(rename_all = "lowercase")]
enum Protocol {
    Rest,
    Ws,
    Fix,
}

struct InputFileParameter {
    /// we keep adding exchanges, no enum
    exchange: String,
    protocol: Protocol,
    format: ApiFileFormat,
}
impl InputFileParameter {
    pub fn from_filename(filename: &str) -> Result<Self, ()> {
        // "binance_ws_asyncapi.yaml"
        if !filename.ends_with(".yaml") {
            return Err(());
        }

        // "binance_ws_asyncapi"
        let rest = filename.replace(".yaml", "");

        // "binance", "ws", "asyncapi"
        let str_vec: Vec<&str> = rest.split_inclusive("_").into_iter().collect();
        if str_vec.len() != 3 {
            return Err(());
        }

        Ok(InputFileParameter {
            exchange: str_vec[0].to_string(),
            protocol: str_vec[1].parse::<Protocol>().map_err(|_e| ())?,
            format: str_vec[2].parse::<ApiFileFormat>().map_err(|_e| ())?,
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

            let files = std::fs::read_dir(input_dir.clone()).unwrap();
            // gather all filenames
            let mut filenames = Vec::new();
            for file in files {
                let file = file.map_err(|_| ())?;
                filenames.push(format!("{}/{:?}", input_dir, file.file_name()));
            }

            let output_directory = cli.output_directory.unwrap();
            for input_filename in filenames {
                for output_language in cli.output_language.clone() {
                    let command = codegen_command_str(
                        input_filename.clone(),
                        output_directory.clone(),
                        output_language,
                    );
                    println!("{:?}", command);
                }
            }
        }
        (Some(input_filename), None) => {
            // single load
            let output_directory = cli.output_directory.unwrap();
            for output_language in cli.output_language {
                let command = codegen_command_str(
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

fn codegen_command_str(
    input_filename: String,
    output_directory: String,
    language: ProgrammingLanguage,
) -> Result<String, ()> {
    // openapi-generator-cli generate -i example_openapi.yaml -g <language> -o output/example_rust_model
    // asyncapi generate models <language> example_asyncapi.yml -o output/example_<language>>_model
    let param = InputFileParameter::from_filename(&input_filename)?;
    let exchange = param.exchange;
    let protocol = param.protocol;
    let format = param.format;
    Ok(match format {
        ApiFileFormat::OpenApi => {
            let output_directory = match language {
                ProgrammingLanguage::Rust => format!(
                    "{}/{:?}/{}/src/{:?}",
                    output_directory, language, exchange, protocol
                ),
                ProgrammingLanguage::Python => format!(
                    "{}/{:?}/{}/{:?}",
                    output_directory, language, exchange, protocol
                ),
            };
            format!("openapi-generator-cli generate -g {language:?} -i {input_filename} -o {output_directory}")
        }
        ApiFileFormat::AsyncApi => {
            let output_directory = match language {
                ProgrammingLanguage::Rust => format!(
                    "{}/{:?}/{}/src/{:?}",
                    output_directory, language, exchange, protocol
                ),
                ProgrammingLanguage::Python => format!(
                    "{}/{:?}/{}/{:?}",
                    output_directory, language, exchange, protocol
                ),
            };
            format!("asyncapi generate models {language:?} {input_filename} -o {output_directory}")
        }
    })
}

pub fn main() {
    match run() {
        Ok(_) => todo!(),
        Err(e) => println!("error detected, {e:?}"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_codegen_command_str() {
        use super::*;
        let input_filename = "binance_ws_asyncapi.yaml".to_string();
        let output_directory = "./".to_string();
        let output_language = ProgrammingLanguage::Rust;
        let command = match codegen_command_str(input_filename, output_directory, output_language) {
            Ok(command) => command,
            Err(e) => panic!("{e:?}"),
        };
        assert_eq!(
            command,
            "asyncapi generate models rust binance_ws_asyncapi.yaml -o ./rust/binance/src/ws"
        )
    }

    #[test]
    fn test_parse_language() {
        use super::*;
        match "python".parse::<ProgrammingLanguage>() {
            Ok(language) => assert_eq!(language, ProgrammingLanguage::Python),
            Err(e) => panic!("{}", e),
        }
    }
}
