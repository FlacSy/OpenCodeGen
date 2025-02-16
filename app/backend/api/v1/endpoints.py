from fastapi import APIRouter, Request
from fastapi.templating import Jinja2Templates

templates = Jinja2Templates(directory='app/frontend/templates')

router = APIRouter()

@router.get("/")
def main_page(request: Request):
    return templates.TemplateResponse("index.html", {"request": request})
    
@router.get("/about")
def about_page(request: Request):
    return templates.TemplateResponse("about.html", {"request": request})
    
@router.get("/contact")
def contact_page(request: Request):
    return templates.TemplateResponse("contact.html", {"request": request})