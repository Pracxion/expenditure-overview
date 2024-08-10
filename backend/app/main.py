
from fastapi import FastAPI

from app.api.api import api

app = FastAPI(openapi_url="/api/v1/openapi.json", docs_url="/api/v1/docs")
app.include_router(api, prefix='/api', tags=['api'])
