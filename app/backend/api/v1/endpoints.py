import os
import json
from ast import literal_eval
from fastapi import APIRouter, Depends, HTTPException, Request, BackgroundTasks
from fastapi.responses import JSONResponse, FileResponse
from fastapi.templating import Jinja2Templates

from app.backend.services.generate_code import CodeGenerator, GeneratedCode, OpenAPI, InputSyntaxError, InputCodeForArchive

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

@router.post('/zip-file')
def zip_file(request_data: InputCodeForArchive, background: BackgroundTasks):
    try:
        if not os.path.exists('temp'):
            os.mkdir('temp')
        
        try:
            openapi_data = json.loads(request_data.openapi)
            zip_archive_name = f"temp/{openapi_data['info']['title']}"
        except (json.JSONDecodeError, KeyError) as e:
            zip_archive_name = "temp/splited_code.zip"

        zip_archive = code_generator.split_file_archive(code=request_data.code, output_archive=zip_archive_name)

        background.add_task(os.remove, zip_archive_name)

        return FileResponse(zip_archive, media_type='application/zip', filename=zip_archive_name)

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
    