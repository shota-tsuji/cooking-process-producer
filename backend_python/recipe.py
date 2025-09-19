import grpc
import logging
from concurrent import futures
#from src.proto.cooking.v1 import process_pb2
#from src.proto.cooking.v1 import process_pb2_grpc
from src.adapter.grpc.generated.proto.cooking.v1 import (
    RecipeWithSchedule, StepWithSchedule, Resource, ProcessServiceBase, CalculateProcessRequest, CalculateProcessResponse
)
from grpc_health.v1 import health, health_pb2_grpc

from grpclib.server import Server
import asyncio
from fastapi import FastAPI
import uvicorn
import threading

import main

class ProcessServiceImpl(ProcessServiceBase):

    async def calculate_process(self, calculate_process_request: CalculateProcessRequest) -> CalculateProcessResponse:
        #recipe_lists = list(map(toRecipeData, request.recipes))

        recipes = [
            RecipeWithSchedule(
                id="1",
                steps=[
                    StepWithSchedule(
                        id="a",
                        duration_minutes=10,
                        resource_id="abc",
                        start_time=1000,
                    )
                ],
            ),
            RecipeWithSchedule(
                id="2",
                steps=[
                    StepWithSchedule(
                        id="b",
                        duration_minutes=20,
                        resource_id="abc",
                        start_time=2000,
                    )
                ],
            ),
        ]

        return CalculateProcessResponse(recipes=recipes)

    def CalculateProcess(self, request, context):
        #recipe_lists = list(map(toRecipeData, request.recipes))

        recipes = [
            process_pb2.RecipeWithSchedule(
                id="1",
                steps=[
                    process_pb2.StepWithSchedule(
                        id="a",
                        duration_minutes=10,
                        resource=process_pb2.Resource(id="abc"),
                        start_time=1000,
                    )
                ],
            ),
            process_pb2.RecipeWithSchedule(
                id="2",
                steps=[
                    process_pb2.StepWithSchedule(
                        id="b",
                        duration_minutes=20,
                        resource=process_pb2.Resource(id="abc"),
                        start_time=2000,
                    )
                ],
            ),
        ]

        return process_pb2.ProcessResponse(recipes=recipes)

#class Greeter(helloworld_pb2_grpc.GreeterServicer):
#
#    def SayHello(self, request, context):
#        print(f'got a request: {request.name}, {request.state}')
#        titles = ["aiko0", "aiko1", "aiko bon"]
#        resource_infos = [helloworld_pb2.ResourceInfo(id=1, amount=2, #isUsedMultiple=True)]
#        return helloworld_pb2.HelloReply(message='Hello, %s!' % request.name, #status=2000, titles=titles,
#                                         resourceInfos=resource_infos)
#
#    def Process(self, request, context):
#        recipe_lists = list(map(toRecipeData, request.recipes))
#        resources = list(map(toResourceData, request.resources))
#
#        stepResults, resource_infos = main.main(recipe_lists, resources)
#        print(stepResults)
#        steps = map(toStepOutput, stepResults)
#        grpc_resource_infos = map(toGrpcResourceInfo, resource_infos)
#
#        return helloworld_pb2.ProcessReply(steps=steps, #resourceInfos=grpc_resource_infos)
#
#
#def toResourceData(grpc_resource):
#    return main.Resource(grpc_resource.id, grpc_resource.amount)
#def toRecipeData(grpc_recipe):
#    steps = list(map(toStepData, grpc_recipe.steps))
#    return main.Recipe(grpc_recipe.id, steps)
#
#def toStepData(grpc_step):
#    return main.RecipeStep(grpc_step.recipe_id, grpc_step.id, grpc_step.duration, #grpc_step.resource_id, grpc_step.order_number)
#
#def toStepOutput(step):
#    return helloworld_pb2.StepOutput(recipe_id=step.recipe_id, step_id=step.step_id, resource_id=step.resource_id, duration=step.duration, start_time=step.start_time,
#                                           time_line_index=step.time_line_index)
#
#def toGrpcResourceInfo(resource_info):
#    return helloworld_pb2.ResourceInfo(id=resource_info.id, amount=resource_info.amount, isUsedMultipleResources=resource_info.isUsedMultipleResources, used_resources_count=resource_info.used_resources_count)

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

#def serve():
#    port = '50051'
#    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
#    #helloworld_pb2_grpc.add_GreeterServicer_to_server(Greeter(), server)
#    process_pb2_grpc.add_ProcessServiceServicer_to_server(ProcessServiceImpl(), server)
#    server.add_insecure_port('[::]:' + port)
#    server.start()
#    print("Server started, listening on " + port)
#    server.wait_for_termination()
#
#
#if __name__ == '__main__':
#    logging.basicConfig()
#    serve()
