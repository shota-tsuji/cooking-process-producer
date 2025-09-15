use crate::domain::dto::step_dto::ScheduledStepDto;

pub struct ScheduledRecipeDto {
    pub recipe_id: String,
    pub steps: Vec<ScheduledStepDto>,
}
