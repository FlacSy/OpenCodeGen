from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

from app.backend.api.v1.endpoints import router

app = FastAPI()

app.include_router(router)
app.mount("/static", StaticFiles(directory="app/frontend/static"), name="static")
