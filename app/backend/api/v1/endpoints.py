from pydantic import BaseModel, HttpUrl
from fastapi import APIRouter, Depends, HTTPException, Request
from fastapi.responses import JSONResponse, RedirectResponse
from fastapi.templating import Jinja2Templates

from app.backend.services.generate_code import CodeGenerator, PythonCode, OpenAPI

templates = Jinja2Templates(directory='app/frontend/templates')

router = APIRouter()

code_generator = CodeGenerator()

@router.post('/generate')
def generate(request_data: OpenAPI):
    try:
        python_code = code_generator.generate(request_data)
        response = {
            "code": 200,
            "status": "success",
            "generated_code": python_code.code
        }
        return JSONResponse(response, status_code=200)
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
    