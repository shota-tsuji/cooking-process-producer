import grpc
from src.adapter.grpc.generated.proto.cooking.v1 import (
    RecipeWithSchedule, StepWithSchedule, Resource, ProcessServiceBase, CalculateProcessRequest, CalculateProcessResponse
)
from src.adapter.grpc import mapper
from grpc_health.v1 import health, health_pb2_grpc

from grpclib.server import Server
import asyncio
from fastapi import FastAPI
import uvicorn
import threading
from collections import defaultdict

import main

def to_proto_calculate_process_response(results: list[main.StepOutput]) -> list[RecipeWithSchedule]:
    """
    Convert a list of StepOutput (domain) into a list of RecipeWithSchedule (proto).
    Groups steps by recipe_id.
    """
    grouped: dict[str, list[main.StepOutput]] = defaultdict(list)
    for s in results:
        grouped[s.recipe_id].append(s)

    recipes_proto: list[RecipeWithSchedule] = []
    for recipe_id, steps in grouped.items():
        steps_proto = [
            StepWithSchedule(
                id=s.step_id,
                duration_minutes=s.duration,
                resource_id=s.resource_id,
                start_time=s.start_time,
            )
            for s in steps
        ]
        recipes_proto.append(RecipeWithSchedule(id=recipe_id, steps=steps_proto))
    return recipes_proto

class ProcessServiceImpl(ProcessServiceBase):

    async def calculate_process(self, calculate_process_request: CalculateProcessRequest) -> CalculateProcessResponse:
        recipes = []
        for recipe in calculate_process_request.recipes:
            recipes.append(mapper.to_domain_recipe(recipe))
        resources = []
        for resource in calculate_process_request.resources:
            resources.append(mapper.to_domain_resource(resource))
        result = main.calculate_process(recipes, resources)
        recipes_with_schedule = to_proto_calculate_process_response(result)
        return CalculateProcessResponse(recipes=recipes_with_schedule)


async def start_server(host: str = "0.0.0.0", port: int = 50051) -> None:
    server = Server([ProcessServiceImpl()])
    await server.start(host, port)
    print("Server started")
    await server.wait_closed()

app = FastAPI()

@app.get("/healthz")
@app.get("/readyz")
async def health():
    # you could add extra checks here (db connection, etc.)
    return {"status": "ok"}


def start_http_server():
    """Run FastAPI (Uvicorn) in its own thread."""
    uvicorn.run(app, host="0.0.0.0", port=8081, log_level="info")

if __name__ == "__main__":
    http_thread = threading.Thread(target=start_http_server, daemon=True)
    http_thread.start()
    asyncio.run(start_server())
