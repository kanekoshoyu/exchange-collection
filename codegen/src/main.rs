use exchange_collection_codegen::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

fn run() -> Result<(), ()> {
    // load
    let cli: CliInput = CliInput::load()?;
    println!("{:#?}", cli);

    // Set up directory
    // copy openapi-generate-ignore file into the target directory
    // Set up Cargo.toml
    // generate
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
                    codegen(
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
            // pre-generate

            let output_directory = cli.output_directory.unwrap();
            for output_language in cli.output_language.clone() {
                codegen(
                    input_filename.clone(),
                    output_directory.clone(),
                    output_language,
                )?;
            }
        }
        _ => unreachable!(),
    }

    // post-generate
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
    let sub_path = PathBuf::from_str(&sub_path_str).map_err(|_| ()).unwrap();
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
) -> Result<Command, ()> {
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

fn codegen(
    input_filename: impl AsRef<Path> + Clone,
    output_directory: impl AsRef<Path>,
    output_language: ProgrammingLanguage,
) -> Result<(), ()> {
    let output_directory =
        codegen_output_directory(input_filename.clone(), output_directory, output_language);

    // precodegen
    {
        // create dir
        match std::fs::create_dir_all(&output_directory) {
            Ok(_) => todo!(),
            Err(e) => println!("failed creating directory, {e}"),
        }

        // copy the ignore script
        let from = PathBuf::from_str(".openapi-generate-ignore").map_err(|_| ())?;
        let to = output_directory.clone();
        match std::fs::copy(from, to) {
            Ok(_) => todo!(),
            Err(e) => println!("failed copying ignore file, {e}"),
        }
    }

    // codegen
    {
        let mut command = codegen_command(input_filename, output_directory, output_language)?;
        println!("{:?}", command);
        match command.output() {
            Ok(o) => println!("codegen success\n{o:?}"),
            Err(e) => println!("codegen fail, {e}"),
        }
    }

    // post_codegen
    {
        // add package info
        match output_language {
            ProgrammingLanguage::Rust => {
                todo!("add mod.rs and Cargo.toml based on the version")
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
    use std::{ffi::OsStr, path::PathBuf, str::FromStr};

    #[test]
    fn test_codegen_command() {
        use super::*;
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
