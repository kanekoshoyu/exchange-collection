use cargo_toml::{InheritedDependencyDetail, Manifest};
use exchange_collection_codegen::*;
use eyre::Result;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use strum::IntoEnumIterator;
#[derive(Clone, Debug)]
pub struct ProtocolCrate {
    pub protocol: Protocol,
    pub version: Version,
}
impl TryFrom<Manifest> for ProtocolCrate {
    type Error = eyre::Error;

    fn try_from(manifest: Manifest) -> std::result::Result<Self, Self::Error> {
        let Some(protocol_str) = manifest.package().name().split("-").last() else {
            return Err(eyre::eyre!("failed parsing protocol name"));
        };
        let version_str = manifest.package().version();
        Ok(ProtocolCrate {
            protocol: protocol_str.parse()?,
            version: version_str.parse()?,
        })
    }
}
impl ProtocolCrate {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        // read Cargo.toml and fill in the data
        let path = path.as_ref();
        let cargo_toml_path = path.join(Path::new("Cargo.toml"));
        let Ok(manifest) = Manifest::from_path(cargo_toml_path) else {
            panic!("failed reading manifest from path");
        };
        ProtocolCrate::try_from(manifest)
    }
}
#[derive(Clone, Debug, Default)]
pub struct ExchangeCrate {
    pub exchange_name: String,
    pub version: Version,
    pub protocol_crates: Vec<ProtocolCrate>,
}
impl ExchangeCrate {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        // read Cargo.toml and fill in the data
        let path = path.as_ref();
        let src_path = path.join(PathBuf::from("src"));

        let protocol_dirs: Vec<PathBuf> = std::fs::read_dir(src_path)?
            .map(|item| item.unwrap().path())
            .filter(|path| path.is_dir())
            .collect();

        let mut protocol_crates = Vec::new();
        for protocol_dir in protocol_dirs {
            protocol_crates.push(ProtocolCrate::from_path(protocol_dir)?);
        }

        let version = protocol_crates
            .iter()
            .cloned()
            .fold(Version::default(), |acc, protocol_crate| {
                acc + protocol_crate.version
            });

        let exchange_name: String = path
            .file_name()
            .unwrap_or_default()
            .to_os_string()
            .into_string()
            .unwrap_or_default();

        Ok(ExchangeCrate {
            exchange_name,
            version,
            protocol_crates,
        })
    }
}

#[derive(Clone, Debug)]
pub struct CollectionCrate {
    pub version: Version,
    pub exchange_crates: Vec<ExchangeCrate>,
}
impl CollectionCrate {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        // read Cargo.toml and fill in the data
        let path = path.as_ref();
        let src_path = path.join(PathBuf::from("src"));

        let exchange_dirs: Vec<PathBuf> = std::fs::read_dir(src_path)?
            .map(|item| item.unwrap().path())
            .filter(|path| path.is_dir())
            .collect();

        let mut exchange_crates = Vec::new();
        for exchange_dir in exchange_dirs {
            exchange_crates.push(ExchangeCrate::from_path(exchange_dir)?);
        }

        let version = exchange_crates
            .iter()
            .cloned()
            .fold(Version::default(), |acc, protocol_crate| {
                acc + protocol_crate.version
            });

        Ok(CollectionCrate {
            version,
            exchange_crates,
        })
    }
}

fn run() -> Result<()> {
    // load
    let cli: CliInput = CliInput::load()?;
    println!("{:#?}", cli);

    // check internet connectivity
    {
        let url = "https://www.google.com";
        if !reqwest::blocking::get(url)?.status().is_success() {
            return Err(eyre::eyre!("the machine is not online"));
        }
    }

    // protocol generation (lowest)
    {
        match (cli.input_filename, cli.input_directory.clone()) {
            (None, Some(input_dir)) => {
                // batch load from input_dir
                println!("batch load");
                let files = std::fs::read_dir(input_dir.clone()).unwrap();
                let mut filenames = Vec::new();
                for file in files {
                    let file = file?;
                    let mut filename = input_dir.clone();
                    filename.push(file.file_name());
                    filenames.push(filename);
                }
                let output_collection_directory = cli.output_directory.to_owned().unwrap();
                for input_filename in filenames {
                    for output_language in cli.output_language.clone() {
                        println!(
                            "generating {} from {:?}",
                            output_language,
                            input_filename.file_name().unwrap()
                        );
                        codegen_protocol_crate(
                            input_filename.clone(),
                            output_collection_directory.clone(),
                            output_language,
                        )?;
                    }
                }
            }
            (Some(input_filename), None) => {
                // single load
                println!("single load");
                let output_directory = cli.output_directory.to_owned().unwrap();
                for output_language in cli.output_language.clone() {
                    codegen_protocol_crate(
                        input_filename.clone(),
                        output_directory.clone(),
                        output_language,
                    )?;
                }
            }
            _ => unreachable!(),
        }
    }

    for language in ProgrammingLanguage::iter() {
        let collection_package_name = "exchange-collection";
        match language {
            ProgrammingLanguage::Rust => {
                let collection_directory = cli
                    .output_directory
                    .to_owned()
                    .unwrap()
                    .join(PathBuf::from_str("rust")?);
                let target_collection_crate =
                    CollectionCrate::from_path(collection_directory.clone())?;
                println!("{:#?}", target_collection_crate);

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
                        manifest.dependencies.insert(
                            target_exchange_crate.exchange_name.clone(),
                            dependency_detail,
                        );
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
                        append_if_missing(
                            collection_directory.join(PathBuf::from_str("src/lib.rs").unwrap()),
                            &format!("pub mod {};", target_exchange_crate.exchange_name),
                        )?;
                    }
                }

                for target_exchange_crate in target_collection_crate.clone().exchange_crates {
                    let dir = format!("src/{}", &target_exchange_crate.exchange_name);
                    let exchange_directory = collection_directory.join(PathBuf::from_str(&dir)?);
                    // exchange Cargo.toml
                    {
                        let target_exchange_crate = target_exchange_crate.clone();
                        let mut manifest: Manifest<()> = cargo_toml::Manifest::default();
                        // assign package
                        let mut exchange_package: cargo_toml::Package<()> =
                            cargo_toml::Package::default();
                        exchange_package.name = target_exchange_crate.exchange_name.clone();
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
                            manifest.dependencies.insert(
                                target_protocol_crate.protocol.to_string(),
                                dependency_detail,
                            );
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
                            append_if_missing(
                                exchange_directory.join(PathBuf::from_str("src/lib.rs")?),
                                &format!("pub mod {};", target_protocol_crate.protocol),
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
            .truncate(true)
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
    let param = InputFileParameter::from_filename(&input_filename).unwrap();
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
fn codegen_protocol_crate_command(
    input_filename: impl AsRef<Path>,
    protocol_directory: impl AsRef<Path>,
    output_language: ProgrammingLanguage,
) -> Result<Command> {
    let param = InputFileParameter::from_filename(&input_filename).unwrap();
    let protocol_directory = protocol_directory.as_ref();
    // output
    let input_filename = input_filename.as_ref();
    Ok(match param.format {
        ApiFileFormat::OpenApi => {
            let mut cmd = Command::new("openapi-generator-cli");
            cmd.arg("generate");
            cmd.arg(format!("-g {}", output_language));
            cmd.arg(format!("-i {}", input_filename.display()));
            cmd.arg(format!("-o {}", protocol_directory.display()));
            cmd.arg("--additional-properties=library=reqwest");
            cmd
        }
        ApiFileFormat::AsyncApi => {
            let mut cmd = Command::new("asyncapi");
            cmd.arg("generate");
            cmd.arg("models");
            cmd.arg(output_language.to_string());
            cmd.arg(format!("{}", input_filename.display()));
            cmd.arg("-o");
            cmd.arg(format!("{}/src", protocol_directory.display()));
            cmd
        }
    })
}

/// codegen for single module, e.g. Binance WS Rust
fn codegen_protocol_crate(
    input_filename: impl AsRef<Path>,
    output_collection_directory: impl AsRef<Path>,
    output_language: ProgrammingLanguage,
) -> Result<()> {
    let param = InputFileParameter::from_filename(&input_filename).unwrap();
    let protocol_directory = output_protocol_directory(
        &input_filename,
        &output_collection_directory,
        output_language,
    );
    println!("codegen_output_directory: {}", protocol_directory.display());
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
                // println!("copying from: {:?}, to: {:?}", from, to);
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
        let mut command =
            codegen_protocol_crate_command(input_filename, &protocol_directory, output_language)?;
        let output = command.output()?;
        let status = output.status;
        match status.success() {
            true => println!("Codegen succeeded"),
            false => println!("Codegen failed"),
        }
    }

    // post-codegen (anything that wraps the codegen material as package)
    {
        // add package info
        match output_language {
            ProgrammingLanguage::Rust => {
                // Cargo.toml
                {
                    let mut manifest: cargo_toml::Manifest<()> = cargo_toml::Manifest::default();
                    let mut package = cargo_toml::Package::default();
                    package.name =
                        format!("exchange-collection-{}-{}", param.exchange, param.protocol);
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
        let command =
            match codegen_protocol_crate_command(input_filename, output_directory, output_language)
            {
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
        assert_eq!(protocol_crate.version, Version::from_str("1.0.0").unwrap());
        assert_eq!(protocol_crate.protocol, Protocol::Rest);
    }

    #[test]
    fn test_exchange_crate_from_dir() {
        let dir = PathBuf::from_str("../target/rust/src/binance").unwrap();
        let protocol_crate = ExchangeCrate::from_path(dir).unwrap();
        assert_eq!(protocol_crate.version, Version::from_str("2.0.0").unwrap());
        assert_eq!(protocol_crate.protocol_crates.len(), 2);
        assert_eq!(protocol_crate.exchange_name, "binance");
    }

    #[test]
    fn test_collecion_crate_from_dir() {
        let dir = PathBuf::from_str("../target/rust").unwrap();
        let protocol_crate = CollectionCrate::from_path(dir).unwrap();
        assert_eq!(protocol_crate.version, Version::from_str("2.0.0").unwrap());
        assert_eq!(protocol_crate.exchange_crates.len(), 1);
    }

    #[test]
    fn test_current_version() {
        let version = Version::current_crate().unwrap();
        assert_eq!(version, Version::from_str("0.1.0").unwrap())
    }
}
