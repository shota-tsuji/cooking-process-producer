use async_trait::async_trait;
use std::error::Error;

#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ProcessRepository: Send + Sync {
    //async fn get_recipe_by_id(&self, resource_id: String) -> Result<Recipe, Box<dyn Error>>;
    async fn register_process(
        &self,
        process_id: String,
        recipe_id_list: Vec<String>,
    ) -> Result<(), Box<dyn Error>>;
}
