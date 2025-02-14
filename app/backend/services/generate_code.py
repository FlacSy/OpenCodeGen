import code_generator
from pydantic import BaseModel

class PythonCode(BaseModel):
    code: str

class OpenAPI(BaseModel):
    openapi: str 

class CodeGenerator:
    def generate(self, openapi: OpenAPI) -> PythonCode:
        openapi_str = str(openapi.openapi)

        python_code: str = str(code_generator.parse_openapi(openapi_str))  # type: ignore
        
        return PythonCode(code=python_code)
