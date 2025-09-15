use crate::domain::{ResourceId, Step};
use std::collections::HashSet;

pub struct Recipe {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<Step>,
}

impl Recipe {
    pub fn resource_ids(&self) -> Vec<ResourceId> {
        self.steps
            .iter()
            .map(|step| step.resource_id.clone())
            .collect::<HashSet<_>>() // remove duplicates
            .into_iter()
            .collect()
    }
}
