use cargo_toml::{InheritedDependencyDetail, Manifest};
use exchange_collection_codegen::meta::*;
use eyre::Result as EyreResult;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Write;
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
    // check internet connectivity
    {
        let url = "https://www.google.com";
        if !reqwest::blocking::get(url)?.status().is_success() {
            return Err(eyre::eyre!("the machine is not online"));
        }
    }

    // code generation (lowest)
    {
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
        let progress_bar = ProgressBar::new((total_files * total_languages) as u64);
        progress_bar.set_style(ProgressStyle::default_bar().template(&template).unwrap());
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
                progress_bar
                    .set_message(format!("{} {}", input_file_param.exchange, output_language)); // Updates the status dynamically
                progress_bar.inc(1);
            }
        }
        progress_bar.finish_with_message("✅ code generation complete");
        if !errors.is_empty() {
            println!("found {} errors as below:", errors.len());
            for error in errors {
                println!("{error}");
            }
        }
    }

    for language in ProgrammingLanguage::iter() {
        let collection_package_name = "exchange-collection";
        match language {
            ProgrammingLanguage::Rust => {
                let collection_directory = output_directory
                    .to_owned()
                    .unwrap()
                    .join(PathBuf::from_str("rust")?);
                let target_collection_crate =
                    CollectionCrate::from_path(collection_directory.clone())?;

                // collection Cargo.toml
                {
                    let collection_crate = target_collection_crate.clone();
                    let mut manifest: Manifest<()> = cargo_toml::Manifest::default();
                    // assign package
                    let mut collection_package: cargo_toml::Package<()> =
                        cargo_toml::Package::default();
                    collection_package.name = collection_package_name.to_string();
                    collection_package.version =
                        cargo_toml::Inheritable::Set(collection_crate.version.to_string());
                    manifest.package = Some(collection_package);
                    // assign dependency
                    for target_exchange_crate in collection_crate.exchange_crates {
                        let exchange_name = target_exchange_crate.exchange_name.clone();
                        let dependency_detail = cargo_toml::DependencyDetail {
                            path: Some(format!("src/{exchange_name}")),
                            version: Some(target_exchange_crate.version.to_string()),
                            ..Default::default()
                        };
                        let dependency_detail = Box::new(dependency_detail);
                        let dependency_detail = cargo_toml::Dependency::Detailed(dependency_detail);
                        let module_name = format!("{}-{}", collection_package_name, exchange_name);
                        manifest.dependencies.insert(module_name, dependency_detail);
                    }
                    // output into a file
                    let manifest_str = toml::to_string(&manifest)?;
                    let cargo_toml = collection_directory
                        .clone()
                        .join(PathBuf::from_str("Cargo.toml").unwrap());
                    std::fs::write(cargo_toml, manifest_str)?;
                }

                // collection lib.rs
                {
                    let collection_crate = target_collection_crate.clone();
                    for target_exchange_crate in collection_crate.exchange_crates {
                        let path =
                            collection_directory.join(PathBuf::from_str("src/lib.rs").unwrap());
                        let module_name = format!(
                            "{}-{}",
                            collection_package_name, target_exchange_crate.exchange_name
                        );
                        append_if_missing(
                            path,
                            &format!("pub use {};", module_name.replace("-", "_")),
                        )?;
                    }
                }

                let mut collection = target_collection_crate.clone();
                collection
                    .exchange_crates
                    .sort_by_key(|e| e.exchange_name.clone());

                for target_exchange_crate in collection.exchange_crates {
                    let dir = format!("src/{}", &target_exchange_crate.exchange_name);
                    let exchange_directory = collection_directory.join(PathBuf::from_str(&dir)?);
                    // exchange Cargo.toml
                    {
                        let target_exchange_crate = target_exchange_crate.clone();
                        let mut manifest: Manifest<()> = cargo_toml::Manifest::default();
                        // assign package
                        let mut exchange_package: cargo_toml::Package<()> =
                            cargo_toml::Package::default();
                        exchange_package.name = format!(
                            "{}-{}",
                            collection_package_name, target_exchange_crate.exchange_name
                        );
                        exchange_package.version =
                            cargo_toml::Inheritable::Set(target_exchange_crate.version.to_string());
                        manifest.package = Some(exchange_package);
                        // assign dependency
                        for target_protocol_crate in target_exchange_crate.protocol_crates {
                            let protocol_name = target_protocol_crate.protocol.to_string();
                            let dependency_detail = cargo_toml::DependencyDetail {
                                path: Some(format!("src/{protocol_name}")),
                                version: Some(target_protocol_crate.version.to_string()),
                                ..Default::default()
                            };
                            let dependency_detail = Box::new(dependency_detail);
                            let dependency_detail =
                                cargo_toml::Dependency::Detailed(dependency_detail);
                            let module_name = format!(
                                "{}-{}-{}",
                                collection_package_name,
                                target_exchange_crate.exchange_name,
                                target_protocol_crate.protocol
                            );
                            manifest.dependencies.insert(module_name, dependency_detail);
                        }
                        // output into a file
                        let manifest_str = toml::to_string(&manifest)?;
                        let cargo_toml = exchange_directory
                            .clone()
                            .join(PathBuf::from_str("Cargo.toml").unwrap());
                        std::fs::write(cargo_toml, manifest_str)?;
                    }

                    // exchange lib.rs
                    {
                        let exchange_crate = target_exchange_crate.clone();
                        for target_protocol_crate in exchange_crate.protocol_crates {
                            let exchange_directory = exchange_directory.clone();
                            let module_name = format!(
                                "{}-{}-{}",
                                collection_package_name,
                                exchange_crate.exchange_name,
                                target_protocol_crate.protocol
                            );
                            append_if_missing(
                                exchange_directory.join(PathBuf::from_str("src/lib.rs")?),
                                &format!("pub use {};", module_name.replace("-", "_")),
                            )
                            .expect("msg");
                        }
                    }
                }
            }
            ProgrammingLanguage::Python => {
                // please implemnent here
            }
        }
    }
    Ok(())
}

/// Appends `line_to_append` to `lib_rs_path` if the line is missing.
fn append_if_missing(lib_rs_path: impl AsRef<Path>, line_to_append: &str) -> std::io::Result<()> {
    let lib_rs_path = lib_rs_path.as_ref();
    // Check if the file exists
    if lib_rs_path.exists() {
        // If the file exists, read the contents of the file
        let content = std::fs::read_to_string(lib_rs_path)?;

        // Check if the line is already present
        if !content.contains(line_to_append) {
            // If not, open the file in append mode and write the line
            let mut file = std::fs::OpenOptions::new().append(true).open(lib_rs_path)?;
            writeln!(file, "{}", line_to_append)?;
        }
    } else {
        // If the file does not exist, create it and write the line
        let mut file = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(lib_rs_path)?;
        writeln!(file, "{}", line_to_append)?;
    }

    Ok(())
}

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
            format!("{}/src/{}/src/{}", output_language, exchange, protocol)
        }
        ProgrammingLanguage::Python => format!("{}/{}/{}", output_language, exchange, protocol),
    };
    let local_protocol_directory = PathBuf::from_str(&local_protocol_directory_str).unwrap();
    // append subpath into the output_directory
    let mut protocol_directory_str = output_collection_directory.as_ref().to_path_buf();
    protocol_directory_str.push(local_protocol_directory);
    PathBuf::from(&protocol_directory_str)
}

/// openapi-generator-cli generate -i example_openapi.yaml -g <language> -o output/example_rust_model
/// asyncapi generate models <language> example_asyncapi.yml -o output/example_<language>>_model
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
        let output = command.output()?;
        let status = output.status;
        // TODO check if we have to do post-codegen upon failure
        if !status.success() {
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
        Ok(_) => println!("success"),
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
        let protocol_crate = ProtocolCrate::from_path(dir).unwrap();
        assert_eq!(protocol_crate.version, Version::from_str("1.1.0").unwrap());
        assert_eq!(protocol_crate.protocol, Protocol::Rest);
    }

    #[test]
    fn test_exchange_crate_from_dir() {
        let dir = PathBuf::from_str("../target/rust/src/binance").unwrap();
        let protocol_crate = ExchangeCrate::from_path(dir).unwrap();
        assert_eq!(protocol_crate.version, Version::from_str("2.2.0").unwrap());
        assert_eq!(protocol_crate.protocol_crates.len(), 2);
        assert_eq!(protocol_crate.exchange_name, "binance");
    }

    #[test]
    fn test_collecion_crate_from_dir() {
        let dir = PathBuf::from_str("../target/rust").unwrap();
        let protocol_crate = CollectionCrate::from_path(dir).unwrap();
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
