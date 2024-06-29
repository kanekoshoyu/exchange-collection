import asyncio
import os
import re
from enum import Enum


class Parser(Enum):
    ASYNC_API = 'asyncapi'
    OPEN_API = 'openapi'


def remove_suffix(text, suffix):
    if text.endswith(suffix):
        return text[:-len(suffix)]
    return text  # Return unchanged if suffix not found or doesn't match


async def generate_rust_models(yaml_file, parser):
    output_dir = "output/"
    if parser == Parser.ASYNC_API:
        output_dir_name = remove_suffix(yaml_file, '_asyncapi.yml')
        command = f"asyncapi generate models rust {yaml_file} -o {output_dir}/{output_dir_name}"
    elif parser == Parser.OPEN_API:
        # todo implement the naming convension
        print("todo implement open API parser")
    else:
        raise ValueError(f"Unknown parser type: {parser}")

    proc = await asyncio.create_subprocess_shell(command)
    await proc.communicate()


async def process_yaml_files(yaml_directory):
    yaml_files = [f for f in os.listdir(yaml_directory) if re.match(
        r'.*(\_asyncapi|\_openapi)\.yml$', f)]

    for yaml_file in yaml_files:
        if yaml_file.endswith('_asyncapi.yml'):
            parser = Parser.ASYNC_API
            print(f"parsing async API yml: {yaml_file}")
        elif yaml_file.endswith('_openapi.yml'):
            parser = Parser.OPEN_API
            print(f"parsing open API yml: {yaml_file}")
        else:
            continue

        await generate_rust_models(os.path.join(yaml_directory, yaml_file), parser)


async def main():
    yaml_directory = './'
    await process_yaml_files(yaml_directory)

if __name__ == "__main__":
    asyncio.run(main())
