import sys
import os
import asyncio
import pytest

sys.path.append(os.path.dirname(os.path.dirname(__file__)))
from recipe import ProcessServiceImpl
from src.adapter.grpc.generated.proto.cooking.v1 import (
    CalculateProcessRequest, RecipeWithSchedule, StepWithSchedule
)

from src.adapter.grpc.generated.proto.cooking.v1 import (
    CalculateProcessRequest, CalculateProcessResponse, Recipe, Step, Resource
)

def make_request2() -> CalculateProcessRequest:
    return CalculateProcessRequest(
        recipes=[
            Recipe(
                id="1",
                steps=[
                    Step(id="1", duration_minutes=2, resource_id="A"),
                    Step(id="2", duration_minutes=1, resource_id="A"),
                    Step(id="3", duration_minutes=1, resource_id="A"),
                ]
            ),
        ],
        resources=[
            Resource(id="A", quantity=1),
        ]
    )

def make_request() -> CalculateProcessRequest:
    return CalculateProcessRequest(
        recipes=[
            Recipe(
                id="1",
                steps=[
                    Step(id="a", duration_minutes=10, resource_id="oven"),
                    Step(id="b", duration_minutes=20, resource_id="oven"),
                ]
            ),
            Recipe(
                id="2",
                steps=[
                    Step(id="c", duration_minutes=15, resource_id="stove"),
                ]
            )
        ],
        resources=[
            Resource(id="oven", quantity=1),
            Resource(id="stove", quantity=1),
        ]
    )

@pytest.mark.asyncio
async def test_calculate_process_returns_expected_response():
    service = ProcessServiceImpl()
    request = make_request()

    response = await service.calculate_process(request)

    assert isinstance(response, CalculateProcessResponse)
    assert len(response.recipes) == 2
    assert response.recipes[0].id == "1"
    assert response.recipes[1].id == "2"
    assert response.recipes[0].steps[0].id == "a"
    assert response.recipes[1].steps[0].id == "c"
    assert response.recipes[0].steps[0].start_time == 0
    assert response.recipes[0].steps[1].start_time == 10
    assert response.recipes[1].steps[0].start_time == 0

@pytest.mark.asyncio
async def test_calculate_process_returns_shorter_time_response():
    service = ProcessServiceImpl()
    request = make_request2()

    response = await service.calculate_process(request)

    assert isinstance(response, CalculateProcessResponse)
    assert len(response.recipes) == 1
    assert response.recipes[0].id == "1"
    assert response.recipes[0].steps[0].id == "1"
    assert response.recipes[0].steps[1].id == "2"
    assert response.recipes[0].steps[0].start_time == 0
    assert response.recipes[0].steps[1].start_time == 2
    assert response.recipes[0].steps[2].start_time == 3
