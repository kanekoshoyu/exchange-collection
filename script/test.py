import unittest
from pathlib import Path
from generate_model import codegen_command, ApiDocFormat, ProgrammingLanguage


class TestCodegenCommand(unittest.TestCase):
    def test_codegen_command(self):
        cmd = codegen_command(Path("./example_asyncapi.yml"), ApiDocFormat.OPENAPI, Path("./output"),
                              ProgrammingLanguage.PYTHON)
        expected_cmd = "openapi-generator-cli generate - i example_asyncapi.yml - g python -o output/example_asyncapi_openapi"
        self.assertEqual(cmd, expected_cmd)


if __name__ == "__main__":
    unittest.main()
