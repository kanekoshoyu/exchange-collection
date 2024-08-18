import unittest
from pathlib import Path
from generate_model import codegen_command, ApiDocFormat, ProgrammingLanguage, extract_exchange_name_from_input_file


class TestCodegenCommand(unittest.TestCase):
    def test_remove_suffix(self):
        exchange = extract_exchange_name_from_input_file(
            "binance_openapi.yaml")
        self.assertEqual(exchange, "binance")
        self.assertEqual(ApiDocFormat.OPENAPI.suffix(), "_openapi.yaml")
        fmt = ApiDocFormat.OPENAPI
        self.assertEqual(fmt.suffix(), "_openapi.yaml")
        exchange = extract_exchange_name_from_input_file(
            "binance_openapi.yaml")
        self.assertEqual(exchange, "binance")

    def test_codegen_command(self):
        cmd = codegen_command(Path("./example_asyncapi.yaml"), ApiDocFormat.OPENAPI, Path("./output"),
                              ProgrammingLanguage.PYTHON)
        print(f"command: {cmd}")
        expected_cmd = "openapi-generator-cli generate -i example_asyncapi.yaml -g python -o output/python/example_openapi"
        self.assertEqual(cmd, expected_cmd)


if __name__ == "__main__":
    unittest.main()
