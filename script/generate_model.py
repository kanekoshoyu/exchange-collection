#!/usr/bin/env python3
import asyncio
from enum import Enum
import argparse
from dataclasses import dataclass
from typing import Optional
from pathlib import Path


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

        # we should only have either single file or a directory
        if args.input_file is not None and args.input_directory is not None:
            raise ValueError(
                "both input file and directory are provided, pick only one")
        if args.input_file is None and args.input_directory is None:
            # default directory as running from the root of the directory
            args.input_directory = "./asset"

        return CliInput(
            input_format=args.input_format,
            input_file=args.input_file,
            input_directory=args.input_directory,
            output_language=args.output_language,
            output_directory=args.output_directory)


class ApiDocFormat(Enum):
    """api doc format, either asyncapi or openapi
    """
    ASYNCAPI = 'asyncapi'
    OPENAPI = 'openapi'

    @classmethod
    def suffix(self) -> str:
        f"_{self.value}.yaml"


class ProgrammingLanguage(Enum):
    """programming language, officially support python and rust in this project
    """
    PYTHON = 'python'
    RUST = 'rust'


def remove_suffix(text, suffix):
    """return the substring after removing suffix"""
    if text.endswith(suffix):
        return text[:-len(suffix)]
    return text


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
#         # todo implement the naming convension
#         print("todo implement open API parser")
#     else:
#         raise ValueError(f"Unknown parser type: {parser}")

def codegen_command(input_file: Path, input_format: ApiDocFormat, output_dir: Path, output_language: ProgrammingLanguage) -> str:
    """cli command for codegen"""
    str_exchange_alias = remove_suffix(input_file.name, input_format.suffix())
    str_output_dir = f"{output_dir.name}/{str_exchange_alias}_{input_format.value}"
    if input_file == ApiDocFormat.ASYNCAPI:
        # asyncapi generate models python example_asyncapi.yml -o output/example_python_model
        f"asyncapi generate models {output_language.value} {input_file.name} -o {str_output_dir}"
    elif input_file == ApiDocFormat.OPENAPI:
        # openapi-generator-cli generate - i path/to/your/openapi.yaml - g < language > -o path/to/output/directory
        f"openapi-generator-cli generate - i {input_file.name} - g {output_language.value} -o {str_output_dir}"
    else:
        raise ValueError(f"Unknown parser type: {input_format}")


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


def run_unit_test():
    import unittest

    class Test(unittest.TestCase):
        def test_codegen_command(self):
            cmd = codegen_command("./", ApiDocFormat.OPENAPI, output_dir="./",
                                  output_language=ProgrammingLanguage.PYTHON)
            self.assertEqual(cmd, "")

    print("running unittest")
    unittest.main()


async def main():
    # OpenAPI
    # 1. scan for all the files to be converted, change to a vector of file names
    # 2. for every file
    # - create output directory, move the .openapi_model_ignore file into the output directory
    # - codegen with `openapi-generator-cli generate -i asset/binance_openapi.yaml -g python -o target/python/binance_openapi`

    cli_input = CliInput.parse_args()
    if cli_input.input_directory:
        # multiple file
        raise NotImplementedError("implement batch loading")
    elif cli_input.input_file:
        # single file
        command = codegen_command(cli_input.input_file, cli_input.input_format,
                                  cli_input.output_directory, cli_input.output_language)
        proc = await asyncio.create_subprocess_shell(command)
        await proc.communicate()
    else:
        raise RuntimeError()

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except Exception as e:
        print(f"Error: {e}")
