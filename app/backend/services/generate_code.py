import json
import code_generator
from enum import Enum
from pydantic import BaseModel

class Language(str, Enum):
    PYTHON = "python"
    PY = "py"
    TYPESCRIPT = "typescript"
    TS = "ts"
    RUST = "rust"
    RS = "rs"
    JAVA = "java"

class GeneratedCode(BaseModel):
    code: str

class OpenAPI(BaseModel):
    openapi: str
    language: Language

    def validate_openapi(self):
        try:
            json.loads(self.openapi)
        except:
            raise InputSyntaxError(f"Invalid OpenAPI JSON")
        return self

class InputSyntaxError(Exception):
    def __init__(self, *args: object) -> None:
        super().__init__(*args)

class CodeGenerator:
    def generate(self, openapi: OpenAPI) -> GeneratedCode:
        openapi_str = openapi.openapi
        language = openapi.language.value

        try:
            generated_code: str = code_generator.generate_code(openapi_str, language)  # type: ignore
        except Exception as e:
            raise ValueError(f"Error generating code: {e}")

        return GeneratedCode(code=generated_code)
