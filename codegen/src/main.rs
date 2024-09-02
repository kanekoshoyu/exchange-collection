use exchange_collection_codegen::*;
use eyre::Result;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use strum::IntoEnumIterator;

fn run() -> Result<()> {
    // load
    let cli: CliInput = CliInput::load()?;
    println!("{:#?}", cli);

    //TODO check internet connection, as nodejs requires internet

    // // exchange-protocol level generation
    // match (cli.input_filename, cli.input_directory) {
    //     (None, Some(input_dir)) => {
    //         // batch load from input_dir
    //         println!("batch load");
    //         let files = std::fs::read_dir(input_dir.clone()).unwrap();
    //         let mut filenames = Vec::new();
    //         for file in files {
    //             let file = file?;
    //             let mut filename = input_dir.clone();
    //             filename.push(file.file_name());
    //             filenames.push(filename);
    //         }
    //         let output_directory = cli.output_directory.unwrap();
    //         for input_filename in filenames {
    //             for output_language in cli.output_language.clone() {
    //                 println!(
    //                     "generating {} from {:?}",
    //                     output_language,
    //                     input_filename.file_name().unwrap()
    //                 );
    //                 codegen_module(
    //                     input_filename.clone(),
    //                     output_directory.clone(),
    //                     output_language,
    //                 )?;
    //             }
    //         }
    //     }
    //     (Some(input_filename), None) => {
    //         // single load
    //         println!("single load");
    //         let output_directory = cli.output_directory.unwrap();
    //         for output_language in cli.output_language.clone() {
    //             codegen_module(
    //                 input_filename.clone(),
    //                 output_directory.clone(),
    //                 output_language,
    //             )?;
    //         }
    //     }
    //     _ => unreachable!(),
    // }

    // exchange level generation
    for language in ProgrammingLanguage::iter() {
        match language {
            ProgrammingLanguage::Rust => {
                // list out all the exchanges
                let paths: Vec<PathBuf> = std::fs::read_dir("target/rust/src")
                    .unwrap()
                    .filter_map(Result::ok)
                    .filter(|entry| entry.path().is_dir())
                    .map(|entry| entry.path())
                    .collect();
                println!("paths: {:?}", paths);
            }
            ProgrammingLanguage::Python => {
                // please implemnent here
            }
        }
    }

    // // crate level generation
    // for language in ProgrammingLanguage::iter() {
    //     match language {
    //         ProgrammingLanguage::Rust => {
    //             // list out all the exchanges
    //             let directories = std::fs::read_dir("target/rust/src")?.into_iter();
    //             for x in directories {
    //                 let x = x?;
    //             }
    //         }
    //         ProgrammingLanguage::Python => {
    //             // please implemnent here
    //         }
    //     }
    // }
    Ok(())
}

fn codegen_module_output_directory(
    input_filename: impl AsRef<Path>,
    output_directory_base: impl AsRef<Path>,
    output_language: ProgrammingLanguage,
) -> PathBuf {
    let param = InputFileParameter::from_filename(&input_filename).unwrap();
    let exchange = param.exchange;
    let protocol = param.protocol;

    // define subpath under output_directory to output
    let sub_path_str = match output_language {
        ProgrammingLanguage::Rust => format!("{}/{}/src/{}", output_language, exchange, protocol),
        ProgrammingLanguage::Python => format!("{}/{}/{}", output_language, exchange, protocol),
    };
    let sub_path = PathBuf::from_str(&sub_path_str).unwrap();
    // append subpath into the output_directory
    let mut output_directory = output_directory_base.as_ref().to_path_buf();
    output_directory.push(sub_path);

    PathBuf::from(&output_directory)
}

/// openapi-generator-cli generate -i example_openapi.yaml -g <language> -o output/example_rust_model
/// asyncapi generate models <language> example_asyncapi.yml -o output/example_<language>>_model
fn codegen_module_command(
    input_filename: impl AsRef<Path>,
    output_directory_base: impl AsRef<Path>,
    output_language: ProgrammingLanguage,
) -> Result<Command> {
    let param = InputFileParameter::from_filename(&input_filename).unwrap();
    let output_directory =
        codegen_module_output_directory(&input_filename, &output_directory_base, output_language);

    // output
    let input_filename = input_filename.as_ref();
    Ok(match param.format {
        ApiFileFormat::OpenApi => {
            let mut cmd = Command::new("openapi-generator-cli");
            cmd.arg("generate");
            cmd.arg(format!("-g {}", output_language));
            cmd.arg(format!("-i {}", input_filename.display()));
            cmd.arg(format!("-o {}", output_directory.display()));
            cmd
        }
        ApiFileFormat::AsyncApi => {
            let mut cmd = Command::new("asyncapi");
            cmd.arg("generate models");
            cmd.arg(format!("{}", output_language));
            cmd.arg(format!("-i {}", input_filename.display()));
            cmd.arg(format!("-o {}", output_directory.display()));
            cmd
        }
    })
}

/// codegen for single module, e.g. Binance WS Rust
fn codegen_module(
    input_filename: impl AsRef<Path>,
    output_directory_base: impl AsRef<Path>,
    output_language: ProgrammingLanguage,
) -> Result<()> {
    let param = InputFileParameter::from_filename(&input_filename).unwrap();
    let output_directory =
        codegen_module_output_directory(&input_filename, &output_directory_base, output_language);
    println!("codegen_output_directory: {}", output_directory.display());
    // pre-codegen (any thing that codegen requires)
    {
        // create dir
        if let Err(e) = std::fs::create_dir_all(&output_directory) {
            return Err(eyre::eyre!("failed creating directory, {e}"));
        }
        match param.format {
            ApiFileFormat::OpenApi => {
                // copy the ignore script into target directory, keep the same filename
                let from = PathBuf::from_str("codegen/.openapi-generator-ignore")?;
                let to = output_directory.clone().join(from.file_name().unwrap());
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
            codegen_module_command(input_filename, &output_directory_base, output_language)?;
        // println!("{:?}", command);
        command.output()?;
    }

    // post-codegen (anything that wraps the codegen material as package)
    {
        // add package info
        match output_language {
            ProgrammingLanguage::Rust => {
                // todo!("add mod.rs and Cargo.toml based on the version");

                // create a new Manifest object, which represents the contents of Cargo.toml
                let mut manifest: cargo_toml::Manifest<()> = cargo_toml::Manifest::default();
                let mut package = cargo_toml::Package::default();
                package.name = format!("exchange-collection-{}-{}", param.exchange, param.protocol);
                package.version = cargo_toml::Inheritable::Set(param.version.to_string());
                package.edition = cargo_toml::Inheritable::Set(cargo_toml::Edition::E2021);
                manifest.package = Some(package);
                let manifest_str = toml::to_string(&manifest)?;
                let cargo_toml = output_directory.join(PathBuf::from_str("Cargo.toml").unwrap());
                std::fs::write(cargo_toml, manifest_str)?;

                // create lib.rs with list of its protocols
                let lib_rs = output_directory
                    .parent()
                    .unwrap()
                    .join(PathBuf::from_str("lib.rs").unwrap());
                append_if_missing(&lib_rs, &format!("pub mod {};", param.protocol))?;

                // anything other than the single module codegen should go to overall_codegen, e.g.
            }
            ProgrammingLanguage::Python => {}
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
            .create(true)
            .write(true)
            .open(lib_rs_path)?;
        writeln!(file, "{}", line_to_append)?;
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
        let output_directory = PathBuf::from_str("target").unwrap();
        let output_language = ProgrammingLanguage::Rust;
        let command =
            match codegen_module_command(input_filename, output_directory, output_language) {
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
    fn test_deserialize_version() {
        // Test a valid version string
        let input_yaml_str = "
            openapi: 3.0.1
            asyncapi: 2.3.0
        ";
        let openapi: OpenApiInfo = serde_yaml::from_str(&input_yaml_str).unwrap();
        let asyncapi: AsyncApiInfo = serde_yaml::from_str(&input_yaml_str).unwrap();
        assert_eq!(
            openapi.version,
            Version {
                major: 3,
                minor: 0,
                patch: 1
            }
        );
        assert_eq!(
            asyncapi.version,
            Version {
                major: 2,
                minor: 3,
                patch: 0
            }
        );
    }
}
