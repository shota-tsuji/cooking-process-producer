from dataclasses import dataclass, field
from typing import List, Optional

@dataclass
class Resource:
    id: str

@dataclass
class Step:
    id: str
    duration_minutes: int
    resource: Resource

@dataclass
class Recipe:
    id: str
    steps: List[Step] = field(default_factory=list)

@dataclass
class StepWithSchedule:
    id: str
    duration_minutes: int
    resource: Resource
    start_time: int  # uint64 in proto, use int in Python

@dataclass
class RecipeWithSchedule:
    id: str
    steps: List[StepWithSchedule] = field(default_factory=list)

