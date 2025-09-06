use crate::domain::resource::Resource;

pub struct Step {
    pub id: String,
    pub description: String,
    pub duration: i32,
    pub resource: Resource,
    pub order: u32,
}