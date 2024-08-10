
from fastapi.routing import APIRouter

api = APIRouter()


@api.get('/', response_model=dict[str, str])
async def hello_world():
    return {"Hellohello": "World"}
