use crate::domain::error::AsyncDynError;
use crate::domain::{Recipe, ScheduledRecipeDto};
use async_trait::async_trait;

#[async_trait]
pub trait ProcessServicePort: Send + Sync {
    async fn calculate_process(
        &self,
        recipes: Vec<Recipe>,
    ) -> Result<Vec<ScheduledRecipeDto>, Box<AsyncDynError>>;
}
