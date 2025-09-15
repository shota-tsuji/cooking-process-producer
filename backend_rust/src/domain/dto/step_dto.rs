use crate::domain::Step;

pub struct ScheduledStepDto {
    pub step: Step,
    pub start_time: u64,
}
