from pydantic import BaseModel, HttpUrl
from fastapi import APIRouter, Depends, HTTPException, Request
from fastapi.responses import JSONResponse, RedirectResponse
from fastapi.templating import Jinja2Templates

from app.backend.services.generate_code import CodeGenerator, GeneratedCode, OpenAPI, InputSyntaxError

templates = Jinja2Templates(directory='app/frontend/templates')

router = APIRouter()

code_generator = CodeGenerator()

@router.post('/generate')
def generate(request_data: OpenAPI):
    try:
        generated_code: GeneratedCode = code_generator.generate(request_data)
        response = {
            "code": 200,
            "status": "success",
            "body": {            
                "generated_code": generated_code.code,
                "language": request_data.language.value
            }
        }
        return JSONResponse(response, status_code=200)
    
    except InputSyntaxError:
        response = {
            "code": 500,
            "status": "error",
            "message": "syntax_error"
        }
        return JSONResponse(response, status_code=500)
    
    except Exception as e:
        response = {
            "code": 500,
            "status": "error",
            "message": str(e)
        }
        return JSONResponse(response, status_code=500)
    
@router.get("/")
def main_page(request: Request):
    return templates.TemplateResponse("index.html", {"request": request})
    