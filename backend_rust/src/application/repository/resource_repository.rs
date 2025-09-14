use crate::domain::Resource;
use async_trait::async_trait;

use crate::domain::error::AsyncDynError;
#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ResourceRepository: Send + Sync {
    async fn get_resource_by_id(&self, resource_id: i32) -> Result<Resource, Box<AsyncDynError>>;
    async fn get_all_resources(&self) -> Result<Vec<Resource>, Box<AsyncDynError>>;
}
