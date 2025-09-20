import sys
import os
import pytest

sys.path.append(os.path.dirname(os.path.dirname(__file__)))
from main import Recipe, RecipeStep, Resource, main, StepOutput


def test_single_recipe_order():
    # Recipe 1: step1 -> step2 -> step3
    steps = [
        RecipeStep(recipe_id="1", step_id="1", duration=2, resource_id="A", order_number=1),
        RecipeStep(recipe_id="1", step_id="2", duration=1, resource_id="A", order_number=2),
        RecipeStep(recipe_id="1", step_id="3", duration=1, resource_id="A", order_number=3),
    ]
    recipe = Recipe(recipe_id="1", steps=steps)
    resources = [Resource(resource_id="A", amount=1)]
    step_outputs = main([recipe], resources)
    expected_order = [
        #StepOutput(resource="A", start=0, end=2, recipe=1, step=1, tli=0),
        #StepOutput(resource="A", start=2, end=3, recipe=1, step=2, tli=0),
        #StepOutput(resource="A", start=3, end=4, recipe=1, step=3, tli=0)
        StepOutput(1, 1, 2, "A", 0),
        StepOutput(1, 2, 1, "A", 2),
        StepOutput(1, 3, 1, "A", 3),
    ]
    print(type(step_outputs[0]), type(expected_order[0]))
    assert step_outputs == expected_order

def test_multiple_recipes_order():
    # Recipe 1: step1 -> step2
    # Recipe 2: step1 -> step2
    steps1 = [
        RecipeStep(recipe_id="1", step_id="1", duration=2, resource_id="A", order_number=1),
        RecipeStep(recipe_id="1", step_id="2", duration=1, resource_id="A", order_number=2),
    ]
    steps2 = [
        RecipeStep(recipe_id="2", step_id="1", duration=1, resource_id="B", order_number=1),
        RecipeStep(recipe_id="2", step_id="2", duration=2, resource_id="B", order_number=2),
    ]
    recipe1 = Recipe(recipe_id="1", steps=steps1)
    recipe2 = Recipe(recipe_id="2", steps=steps2)
    resources = [Resource(resource_id="A", amount=1), Resource(resource_id="B", amount=1)]
    step_outputs = main([recipe1, recipe2], resources)
    # Each recipe's steps must be in order, but recipes may interleave
    expected_order = [
        StepOutput(1, 1, 2, "A", 0),
        StepOutput(1, 2, 1, "A", 2),
        StepOutput(2, 1, 1, "B", 0),
        StepOutput(2, 2, 2, "B", 1),
    ]
    assert step_outputs == expected_order

def test_when_resource_contention_then_shorter_time_proposed():
    steps1 = [
        RecipeStep(recipe_id="1", step_id="1", duration=1, resource_id="A", order_number=1),
        RecipeStep(recipe_id="1", step_id="2", duration=1, resource_id="C", order_number=2),
    ]
    steps2 = [
        RecipeStep(recipe_id="2", step_id="1", duration=2, resource_id="A", order_number=1),
        RecipeStep(recipe_id="2", step_id="2", duration=2, resource_id="D", order_number=2),
    ]
    recipe1 = Recipe(recipe_id="1", steps=steps1)
    recipe2 = Recipe(recipe_id="2", steps=steps2)
    resources = [
        Resource(resource_id="A", amount=1),
        Resource(resource_id="B", amount=1),
        Resource(resource_id="C", amount=1),
        Resource(resource_id="D", amount=1)
    ]

    step_outputs = main([recipe1, recipe2], resources)

    expected_order = [
        StepOutput(1, 1, 1, "A", 2),
        StepOutput(1, 2, 1, "C", 3),
        StepOutput(2, 1, 2, "A", 0),
        StepOutput(2, 2, 2, "D", 2),
    ]
    assert step_outputs == expected_order

def test_parallel_steps_with_multiple_resources():
    # Two steps can run in parallel due to two resources
    steps = [
        RecipeStep(recipe_id="1", step_id="1", duration=2, resource_id="A", order_number=1),
        RecipeStep(recipe_id="1", step_id="2", duration=2, resource_id="A", order_number=2),
    ]
    recipe = Recipe(recipe_id="1", steps=steps)
    resources = [Resource(resource_id="A", amount=2)]
    step_outputs = main([recipe], resources)
    print("result: ", step_outputs)

    expected_order = [
        StepOutput(1, 1, 2, "A", 0),
        StepOutput(1, 2, 2, "A", 2),
    ]

    assert step_outputs == expected_order

if __name__ == "__main__":
    pytest.main([__file__])

