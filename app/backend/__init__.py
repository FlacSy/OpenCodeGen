from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

from app.backend.api.v1.endpoints import router
from app.backend.api.v1.api import router as api_router

app = FastAPI()

app.include_router(router)
app.include_router(api_router)
app.mount("/static", StaticFiles(directory="app/frontend/static"), name="static")
