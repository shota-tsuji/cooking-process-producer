use crate::domain::ResourceId;

#[derive(Default)]
pub struct Step {
    pub id: String,
    pub description: String,
    pub duration: i32,
    pub order: u32,
    pub resource_id: ResourceId,
}
