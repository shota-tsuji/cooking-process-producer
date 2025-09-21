"""
Mapper functions to convert between gRPC v1 messages and the domain model.
"""

#from src.adapter.grpc.generated.proto.cooking.v1 import Recipe as V1Recipe, Step as V1Step
from src.adapter.grpc.generated.proto.cooking.v1 import (
    Recipe as V1Recipe,
    Step as V1Step,
    Resource as V1Resource,
)
#from main import Recipe as DomainRecipe, RecipeStep as DomainRecipeStep
from main import (
    Recipe as DomainRecipe,
    RecipeStep as DomainRecipeStep,
    Resource as DomainResource,
)


def to_domain_recipe(proto_recipe: V1Recipe) -> DomainRecipe:
    """
    Convert a gRPC v1 Recipe message into the domain Recipe.
    """
    steps = []
    for i, step in enumerate(proto_recipe.steps):
        steps.append(to_domain_step(proto_recipe.id, step, i))
    return DomainRecipe(id=proto_recipe.id, steps=steps)


def to_domain_step(recipe_id: str, proto_step: V1Step, order: int) -> DomainRecipeStep:
    """
    Convert a gRPC v1 Step message into the domain RecipeStep.
    """
    # Domain uses 'duration' instead of 'duration_minutes'
    # and requires order_number. We default to 0 if order is
    # not provided in the proto definition.
    return DomainRecipeStep(
        recipe_id=recipe_id,
        step_id=proto_step.id,
        duration=proto_step.duration_minutes,
        resource_id=proto_step.resource_id,
        order_number=order
    )

def to_domain_resource(proto_resource: V1Resource) -> DomainResource:
    """
    Convert a gRPC v1 Resource message into the domain Resource.
    """
    return DomainResource(
        resource_id=proto_resource.id,
        amount=proto_resource.quantity,
    )