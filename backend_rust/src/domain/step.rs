use crate::domain::resource::Resource;

pub struct Step {
    pub id: i32,
    pub description: String,
    pub duration: i32,
    pub resource: Resource,
    pub order: i32,
}