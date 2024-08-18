#!/usr/bin/env python3
import asyncio
from enum import Enum
import argparse
from dataclasses import dataclass
from typing import Optional
from pathlib import Path
import re
import shutil


# TODO allow a vector of output language


@dataclass
class CliInput:
    """
    input_format: asyncapi or openapi or none
    output_language: python or rust or none(both)

    """
    input_format: Optional[str]
    input_file: Optional[str]
    input_directory: Optional[str]
    output_language: Optional[str]
    output_directory: Optional[str]
    test: bool = False

    @classmethod
    def parse_args(self):
        parser = argparse.ArgumentParser(
            description="Process CLI inputs for code generation.")

        parser.add_argument("-f", "--input_format", type=str, choices=["asyncapi", "openapi", "none"],
                            help="Specify the input format: asyncapi, openapi, or none.")
        parser.add_argument("-if", "--input_file", type=str,
                            help="Path to the input file.")
        parser.add_argument("-id", "--input_directory", type=str,
                            help="Path to the input directory.")
        parser.add_argument("-ol", "--output_language", type=str, choices=["python", "rust"],
                            help="Specify the output language: python or rust.")
        parser.add_argument("-od", "--output_directory", type=str,
                            help="Path to the output directory.")

        args = parser.parse_args()

        input = CliInput(
            input_format=args.input_format,
            input_file=args.input_file,
            input_directory=args.input_directory,
            output_language=args.output_language,
            output_directory=args.output_directory)

        # we should only have either single file without format or a directory with input format
        if input.input_file and input.input_directory:
            raise ValueError(
                "both input file and directory are provided, pick only one")
        if not input.input_file and not input.input_directory:
            # default directory as running from the root of the directory
            input.input_directory = "./asset"
        # fill the default values for different cases
        if not input.input_format and input.input_file:
            for format in ApiDocFormat:
                if input.input_file.__contains__(format.suffix()):
                    input.input_format = format
        if not input.output_directory:
            input.output_directory = "."
        return input


class ApiDocFormat(Enum):
    """api doc format, either asyncapi or openapi
    """
    ASYNCAPI = 'asyncapi'
    OPENAPI = 'openapi'

    def suffix(self) -> str:
        return f"_{self.value}.yaml"


class ProgrammingLanguage(Enum):
    """programming language, officially support python and rust in this project
    """
    PYTHON = 'python'
    RUST = 'rust'


# def get_exchange_name_from_file(text: str, suffix: str):
#     """return the substring after removing suffix"""
#     if text.endswith(suffix):
#         return text[:-len(suffix)]
#     return text


def extract_exchange_name_from_input_file(filename: str) -> str:
    # Convert the filename to a Path object to normalize it
    path = Path(filename)

    # Use regex to match and extract the exchange name
    if match := re.search(r'([^/\\]+)_(openapi|asyncapi)\.yaml$', path.name):
        exchange_name = match.group(1)  # Extract the exchange name
        return exchange_name
    else:
        raise ValueError(
            f"Filename '{filename}' does not match the expected pattern.")


# async def generate_model_froms_single_api_doc(yaml_file, parser):
#     # for testing
#     # output_dir = "output/"
#     # for production
#     languages = [ProgrammingLanguage.PYTHON, ProgrammingLanguage.RUST]
#     if parser == ApiDocFormat.ASYNCAPI:
#         output = remove_suffix(yaml_file, '_asyncapi.yml')
#         for language in languages:
#             output_dir = f"target/{language.value}/{output}/model"
#             command = f"asyncapi generate models {language.value} {yaml_file} -o {output_dir}"
#             print(f"command: {command}")
#             proc = await asyncio.create_subprocess_shell(command)
#             await proc.communicate()
#     elif parser == ApiDocFormat.OPENAPI:
#         # TODO implement the naming convension
#         print("TODO implement open API parser")
#     else:
#         raise ValueError(f"Unknown parser type: {parser}")


def output_directory(input_file: Path, input_format: ApiDocFormat, output_dir: Path, output_language: ProgrammingLanguage) -> Path:
    str_input_file = str(input_file)
    str_exchange_alias = extract_exchange_name_from_input_file(str_input_file)
    return Path(f"{str(output_dir)}/{output_language.value}/{str_exchange_alias}_{input_format.value}")


def codegen_command(input_file: Path, input_format: ApiDocFormat, output_dir: Path, output_language: ProgrammingLanguage) -> str:
    """cli command for codegen"""
    str_input_file = str(input_file)
    str_output_dir = str(output_directory(
        input_file, input_format, output_dir, output_language))
    if input_format == ApiDocFormat.ASYNCAPI:
        # asyncapi generate models python example_asyncapi.yml -o output/example_python_model
        return f"asyncapi generate models {output_language.value} {str_input_file} -o {str_output_dir}"
    elif input_format == ApiDocFormat.OPENAPI:
        # openapi-generator-cli generate - i path/to/your/openapi.yaml - g < language > -o path/to/output/directory
        return f"openapi-generator-cli generate -i {str_input_file} -g {output_language.value} -o {str_output_dir}"


async def run_command(command: str):
    import subprocess
    # Create the subprocess with stdout and stderr redirected to PIPE
    proc = await asyncio.create_subprocess_shell(
        command,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE
    )

    # Wait for the subprocess to finish and capture the output and errors
    stdout, stderr = await proc.communicate()

    # If needed, you can process the captured stdout and stderr here
    # Example: Convert stdout and stderr to strings
    stdout_str = stdout.decode()
    stderr_str = stderr.decode()

    # If you want to log or handle the output, you can do so here
    # For now, we just return the output and error
    return stdout_str, stderr_str

# async def process_yaml_files(yaml_directory):
#     yaml_files = [f for f in os.listdir(yaml_directory) if re.match(
#         r'.*(\_asyncapi|\_openapi)\.yml$', f)]

#     for yaml_file in yaml_files:
#         if yaml_file.endswith('_asyncapi.yml'):
#             parser = ApiDocFormat.ASYNCAPI
#             print(f"parsing async API yml: {yaml_file}")
#         elif yaml_file.endswith('_openapi.yml'):
#             parser = ApiDocFormat.OPENAPI
#             print(f"parsing open API yml: {yaml_file}")
#         else:
#             continue

#         await codegen_command(os.path.join(yaml_directory, yaml_file), parser)


async def main():
    # arguments
    cli_input = CliInput.parse_args()
    print(cli_input)

    if cli_input.input_directory:
        # multiple file
        raise NotImplementedError("implement batch loading")
    elif cli_input.input_file:
        # single input, sigle output
        if cli_input.output_language:
            print("generating one language")

            command = codegen_command(cli_input.input_file, cli_input.input_format,
                                      cli_input.output_directory, output_language)

            # create output directory, copy .openapi_model_ignore file into the output directory
            if cli_input.input_format == ApiDocFormat.OPENAPI:
                output_dir = output_directory(cli_input.input_file, cli_input.input_format,
                                              cli_input.output_directory, output_language)
                output_dir.mkdir(parents=True, exist_ok=True)
                # TODO make this programmable as well
                source_file = Path("./config/.openapi-generator-ignore")
                target_file = output_dir / source_file.name
                shutil.copy(source_file, target_file)

            print(f"running: {command}")
            await run_command(command)
        else:
            print("generating per language")
            # single input, multiple output
            for output_language in ProgrammingLanguage:
                command = codegen_command(cli_input.input_file, cli_input.input_format,
                                          cli_input.output_directory, output_language)

                # create output directory, copy .openapi_model_ignore file into the output directory
                if cli_input.input_format == ApiDocFormat.OPENAPI:
                    output_dir = output_directory(cli_input.input_file, cli_input.input_format,
                                                  cli_input.output_directory, output_language)
                    output_dir.mkdir(parents=True, exist_ok=True)
                    # TODO make this programmable as well
                    source_file = Path("./config/.openapi-generator-ignore")
                    target_file = output_dir / source_file.name
                    shutil.copy(source_file, target_file)

                print(f"running: {command}")
                await run_command(command)
    else:
        raise RuntimeError()

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except Exception as e:
        print(f"Error: {e}")
