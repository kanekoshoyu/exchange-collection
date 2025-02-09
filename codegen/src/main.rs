use cargo_toml::InheritedDependencyDetail;
use exchange_collection_codegen::meta::*;
use eyre::Result as EyreResult;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use strum::IntoEnumIterator;

fn run() -> EyreResult<()> {
    // load
    let CliInput {
        input_directory,
        input_filename,
        output_directory,
        output_languages,
    } = CliInput::load()?;

    println!("[1/3] check internet connection");
    {
        let url = "https://www.google.com";
        if !reqwest::blocking::get(url)?.status().is_success() {
            return Err(eyre::eyre!("the machine is not online"));
        }
    }

    println!("[2/3] code generation per exchange protocol");
    {
        // either 1 file speficied or all files in the directory
        let input_files_param = match (input_filename, input_directory.clone()) {
            // batch load from input_dir
            (None, Some(input_dir)) => InputFileParameter::from_directory(&input_dir)?,
            (Some(input_filename), None) => {
                vec![InputFileParameter::from_file_path(input_filename)?]
            }
            _ => unreachable!(),
        };

        let output_collection_directory = output_directory.to_owned().unwrap();
        let total_languages = output_languages.len();
        let total_files = &input_files_param.len();
        let template = {
            let max_exchange_name_len = input_files_param
                .iter()
                .map(|i| i.exchange.len())
                .max()
                .unwrap();
            let max_language_name_len = output_languages
                .iter()
                .map(|i| i.to_string().len())
                .max()
                .unwrap();
            let max_message_len = max_exchange_name_len + max_language_name_len + 1;
            format!("[{{elapsed_precise}}] (eta: {{eta:4}}) {{wide_bar}} {{pos:>3}}/{{len:3}} {{msg:{max_message_len}}}")
        };
        let child_progress = ProgressBar::new((total_files * total_languages) as u64);
        child_progress.set_style(ProgressStyle::default_bar().template(&template).unwrap());
        let mut errors = Vec::new();
        for input_file_param in input_files_param {
            for output_language in output_languages.clone() {
                if let Err(e) = codegen_protocol_crate(
                    input_file_param.filename.clone(),
                    output_collection_directory.clone(),
                    output_language,
                    &input_file_param.exchange,
                ) {
                    errors.push(e);
                };
                child_progress
                    .set_message(format!("{} {}", input_file_param.exchange, output_language)); // Updates the status dynamically
                child_progress.inc(1);
            }
        }
        child_progress.finish_and_clear();
        if !errors.is_empty() {
            println!("found {} errors as below:", errors.len());
            for error in errors {
                println!("{error}");
            }
        }
    }

    println!("[3/3] package as workspace");
    for language in ProgrammingLanguage::iter() {
        match language {
            ProgrammingLanguage::Rust => {
                let workspace_directory = output_directory
                    .to_owned()
                    .unwrap()
                    .join(PathBuf::from_str("rust")?);
                let workspace_src_directory = workspace_directory.join("src");
                let collection_workspace =
                    CollectionWorkspace::from_src_path(&workspace_src_directory)?;

                // workspace Cargo.toml
                {
                    let workspace_toml = collection_workspace.to_toml();
                    // output into a file
                    let workspace_toml_str = toml::to_string(&workspace_toml)?;
                    // Manually add `[workspace]` as its missing
                    let workspace_toml_str = format!("[workspace]\n{}", workspace_toml_str);
                    let cargo_toml = workspace_src_directory
                        .parent()
                        .unwrap()
                        .join(PathBuf::from_str("Cargo.toml").unwrap());
                    std::fs::write(cargo_toml, workspace_toml_str)?;
                }
            }
            ProgrammingLanguage::Python => {
                // please implemnent here
            }
        }
    }
    println!("code generation complete");
    Ok(())
}

/// rust: project content directory
fn output_protocol_directory(
    input_filename: impl AsRef<Path>,
    output_collection_directory: impl AsRef<Path>,
    output_language: ProgrammingLanguage,
) -> PathBuf {
    let param = InputFileParameter::from_file_path(&input_filename).unwrap();
    let exchange = param.exchange;
    let protocol = param.protocol;

    // define subpath under output_directory to output
    let local_protocol_directory_str = match output_language {
        ProgrammingLanguage::Rust => {
            format!("{}/src/{}_{}", output_language, exchange, protocol)
        }
        ProgrammingLanguage::Python => format!("{}/{}/{}", output_language, exchange, protocol),
    };
    let local_protocol_directory = PathBuf::from_str(&local_protocol_directory_str).unwrap();
    // append subpath into the output_directory
    let mut protocol_directory_str = output_collection_directory.as_ref().to_path_buf();
    protocol_directory_str.push(local_protocol_directory);
    PathBuf::from(&protocol_directory_str)
}

/// openapi-generator-cli generate -i {example_openapi.yaml} -g <language> -o {output_dir}
/// asyncapi generate fromTemplate {example_asyncapi.yml} asyncapi-rust-ws-template -p exchange={rust} -o {output_dir}
fn command_for_codegen_protocol_crate(
    input_filename: impl AsRef<Path>,
    output_protocol_crate_directory: impl AsRef<Path>,
    output_language: ProgrammingLanguage,
    exchange_name: &str,
) -> EyreResult<Command> {
    let param = InputFileParameter::from_file_path(&input_filename).unwrap();
    let out_dir = output_protocol_crate_directory.as_ref();
    // output
    let input_filename = input_filename.as_ref();
    Ok(match param.format {
        ApiFileFormat::OpenApi => {
            let mut cmd = Command::new("openapi-generator-cli");
            cmd.arg("generate");
            cmd.arg(format!("-g {}", output_language));
            cmd.arg(format!("-i {}", input_filename.display()));
            cmd.arg(format!("-o {}", out_dir.display()));
            cmd.arg("--additional-properties=library=reqwest");
            cmd
        }
        ApiFileFormat::AsyncApi => {
            // asyncapi generate fromTemplate asset/okx_ws_asyncapi.yaml asyncapi-rust-ws-template -p exchange=okx -o ~/Desktop/okx
            let mut cmd = Command::new("asyncapi");
            cmd.arg("generate");
            cmd.arg("fromTemplate");
            cmd.arg(format!("{}", input_filename.display()));
            cmd.arg("asyncapi-rust-ws-template");
            cmd.arg("-p");
            cmd.arg(format!("exchange={}", exchange_name));
            cmd.arg("-o");
            cmd.arg(format!("{}", out_dir.display()));
            cmd.arg("--force-write");
            cmd
        }
    })
}

/// codegen for single module, e.g. Binance WS Rust
fn codegen_protocol_crate(
    input_filename: impl AsRef<Path>,
    output_collection_directory: impl AsRef<Path>,
    output_language: ProgrammingLanguage,
    exchange_name: &str,
) -> EyreResult<()> {
    let param = InputFileParameter::from_file_path(&input_filename).unwrap();
    // frankly the whole thing about protocol directory can be removed

    let protocol_directory = output_protocol_directory(
        &input_filename,
        &output_collection_directory,
        output_language,
    );
    // pre-codegen (any thing that codegen requires)
    {
        // create dir
        if let Err(e) = std::fs::create_dir_all(&protocol_directory) {
            return Err(eyre::eyre!("failed creating directory, {e}"));
        }
        match param.format {
            ApiFileFormat::OpenApi => {
                // copy the ignore script into target directory, keep the same filename
                let from = PathBuf::from_str("codegen/.openapi-generator-ignore")?;
                let to = protocol_directory.clone().join(from.file_name().unwrap());
                if let Err(e) = std::fs::copy(from, to) {
                    return Err(eyre::eyre!("failed copying ignore file, {e}"));
                }
            }
            ApiFileFormat::AsyncApi => {
                // TODO add asyncapi ignore file etc
            }
        }
    }

    // codegen
    {
        let mut command = command_for_codegen_protocol_crate(
            input_filename,
            &protocol_directory,
            output_language,
            exchange_name,
        )?;
        // execute and await output
        let output = command.output()?;
        // TODO check if we have to do post-codegen upon failure
        if !output.status.success() {
            return Err(eyre::eyre!("codegen failed, {:?}", command));
        }
    }

    // post-codegen (anything that wraps the codegen material as package)
    {
        // add package info
        match output_language {
            ProgrammingLanguage::Rust => {
                // Cargo.toml
                {
                    let collection_package_name = "exchange-collection";
                    let mut manifest: cargo_toml::Manifest<()> = cargo_toml::Manifest::default();
                    let mut package = cargo_toml::Package::default();
                    package.name = format!(
                        "{}-{}-{}",
                        collection_package_name, param.exchange, param.protocol
                    );
                    // version of a generated protocol is the sum of codegen version and config version
                    let version = param.version + Version::current_crate()?;
                    package.version = cargo_toml::Inheritable::Set(version.to_string());
                    package.edition = cargo_toml::Inheritable::Set(cargo_toml::Edition::E2021);
                    manifest.package = Some(package);

                    let dependencies = ["reqwest", "serde", "serde_json", "serde_yaml", "url"];
                    for dependecy in dependencies {
                        manifest.dependencies.insert(
                            dependecy.to_string(),
                            cargo_toml::Dependency::Inherited(InheritedDependencyDetail {
                                workspace: true,
                                ..Default::default()
                            }),
                        );
                    }
                    // output as a file
                    std::fs::write(
                        protocol_directory.join(PathBuf::from_str("Cargo.toml").unwrap()),
                        toml::to_string(&manifest)?,
                    )?;
                }
                // anything other than the single module codegen should go to overall_codegen, e.g.
            }
            ProgrammingLanguage::Python => {}
        }
    }
    Ok(())
}

pub fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => println!("error detected, {e:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ffi::OsStr, path::PathBuf, str::FromStr};

    #[test]
    fn test_codegen_command() {
        let input_filename = PathBuf::from_str("../asset/binance_ws_asyncapi.yaml").unwrap();
        let output_directory = PathBuf::from_str("target/rust/src/binance/src/ws").unwrap();
        let output_language = ProgrammingLanguage::Rust;
        let command = match command_for_codegen_protocol_crate(
            input_filename,
            output_directory,
            output_language,
            "binance",
        ) {
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
                "generate",
                "models",
                "rust",
                "../asset/binance_ws_asyncapi.yaml",
                "-o",
                "target/rust/src/binance/src/ws/src"
            ]
            .to_vec()
        );
    }

    #[test]
    fn test_deserialize_language() {
        match ProgrammingLanguage::from_str("python") {
            Ok(language) => assert_eq!(language, ProgrammingLanguage::Python),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_serialize_language() {
        let language = ProgrammingLanguage::Python;
        assert_eq!(format!("{language}"), "python");
    }

    #[test]
    fn test_parse_input_file() {
        match InputFileParameter::from_file_path("../asset/binance_ws_asyncapi.yaml") {
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
        // ends_with() does not work well with PathBuf
    }

    #[test]
    fn test_copy() {
        // TODO use a temp dir
        // input is a file
        let str_path = "../asset/binance_ws_asyncapi.yaml";
        // output has to be a file as well, do not pass directory
        let to = PathBuf::from_str("./").unwrap();
        let to = to.join(PathBuf::from(str_path).file_name().unwrap());
        if let Err(e) = std::fs::copy(str_path, to) {
            panic!("error: {e}");
        }
    }

    #[test]
    fn test_protocol_crate_from_dir() {
        let dir = PathBuf::from_str("../target/rust/src/binance/src/rest")
            .unwrap()
            .canonicalize()
            .unwrap();
        let protocol_crate = ExchangeProtocolCrate::from_path(dir).unwrap();
        assert_eq!(protocol_crate.version, Version::from_str("1.1.0").unwrap());
        assert_eq!(protocol_crate.protocol, Protocol::Rest);
    }

    #[test]
    fn test_collecion_crate_from_dir() {
        let dir = PathBuf::from_str("../target/rust").unwrap();
        let protocol_crate = CollectionWorkspace::from_src_path(dir).unwrap();
        assert_eq!(protocol_crate.version, Version::from_str("2.2.0").unwrap());
        assert_eq!(protocol_crate.exchange_crates.len(), 1);
    }

    #[test]
    fn test_current_version() {
        let version = Version::current_crate().unwrap();
        assert_eq!(version, Version::from_str("0.1.0").unwrap())
    }

    // TODO: generate a new test_target and check if the generated target is in the right format
}
