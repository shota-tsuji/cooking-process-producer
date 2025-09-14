use crate::domain::Step;
use crate::domain::dto::resource_dto::ResourceDto;

pub struct ScheduledStepDto {
    pub step: Step,
    pub start_time: u64,
    pub resource: ResourceDto,
}
