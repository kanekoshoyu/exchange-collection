use exchange_collection_codegen::*;
use eyre::Result;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

fn run() -> Result<()> {
    // load
    let cli: CliInput = CliInput::load()?;
    println!("{:#?}", cli);

    //TODO check internet connection, as nodejs requires internet

    // pick input/output files accordingly
    match (cli.input_filename, cli.input_directory) {
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

            let output_directory = cli.output_directory.unwrap();
            for input_filename in filenames {
                for output_language in cli.output_language.clone() {
                    codegen_module(
                        input_filename.clone(),
                        output_directory.clone(),
                        output_language,
                    )?;
                }
            }
        }
        (Some(input_filename), None) => {
            // single load
            println!("single load");
            let output_directory = cli.output_directory.unwrap();
            for output_language in cli.output_language.clone() {
                codegen_module(
                    input_filename.clone(),
                    output_directory.clone(),
                    output_language,
                )?;
            }
        }
        _ => unreachable!(),
    }

    // post-codegen, i.e. package level things like version of a module and package

    Ok(())
}

fn codegen_output_directory(
    input_filename: impl AsRef<Path>,
    output_directory: impl AsRef<Path>,
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
    let mut output_directory = output_directory.as_ref().to_path_buf();
    output_directory.push(sub_path);

    PathBuf::from(&output_directory)
}

/// openapi-generator-cli generate -i example_openapi.yaml -g <language> -o output/example_rust_model
/// asyncapi generate models <language> example_asyncapi.yml -o output/example_<language>>_model
fn codegen_command(
    input_filename: impl AsRef<Path>,
    output_directory: impl AsRef<Path>,
    output_language: ProgrammingLanguage,
) -> Result<Command> {
    let param = InputFileParameter::from_filename(&input_filename).unwrap();
    let output_directory =
        codegen_output_directory(&input_filename, &output_directory, output_language);
    // let output_directory = output_directory.canonicalize().unwrap();

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

/// codegen for single module, e.g. Binance WS
fn codegen_module(
    input_filename: impl AsRef<Path>,
    output_directory_base: impl AsRef<Path>,
    output_language: ProgrammingLanguage,
) -> Result<()> {
    let param = InputFileParameter::from_filename(&input_filename).unwrap();
    let output_directory =
        codegen_output_directory(&input_filename, &output_directory_base, output_language);

    // pre-codegen (any thing that codegen requires)
    {
        // create dir
        if let Err(e) = std::fs::create_dir_all(&output_directory) {
            println!("failed creating directory, {e}");
        }

        // copy the ignore script into target directory, keep the same filename
        let from = PathBuf::from_str(".openapi-generate-ignore")?;
        let to = output_directory.clone().join(from.file_name().unwrap());
        if let Err(e) = std::fs::copy(from, to) {
            println!("failed copying ignore file, {e}");
        }
    }

    // codegen
    {
        let mut command = codegen_command(input_filename, &output_directory, output_language)?;
        println!("{:?}", command);
        match command.output() {
            Ok(o) => println!("codegen success\n{o:?}"),
            Err(e) => println!("codegen fail, {e}"),
        }
    }

    // post-codegen (anything that wraps the codegen material as package)
    {
        // add package info
        match output_language {
            ProgrammingLanguage::Rust => {
                // todo!("add mod.rs and Cargo.toml based on the version");
                let cargo_toml = output_directory.join(PathBuf::from_str("Cargo.toml").unwrap());
                let contents = format!(
                    r#"
                [package]
                name = "exchange-collection-{}"
                version = "{}"
                edition = "2021"

                [lib]
                "#,
                    param.exchange, param.version
                );
                std::fs::write(cargo_toml, contents)?;

                let lib_rs = output_directory.join(PathBuf::from_str("lib.rs").unwrap());
                let contents = format!("pub use {};", param.protocol);
                std::fs::write(lib_rs, contents)?;
                // anything other than the single codegen should go to overall_codegen
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
        let output_directory = PathBuf::from_str("target").unwrap();
        let output_language = ProgrammingLanguage::Rust;
        let mut command = match codegen_command(input_filename, output_directory, output_language) {
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
        let res = command.output();
        match res {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
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
