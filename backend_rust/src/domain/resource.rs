#[derive(Debug, Clone)]
pub struct Resource {
    pub id: i32,
    pub name: String,
    pub amount: i32,
}

impl Resource {
    pub fn new(id: i32, name: String, amount: i32) -> Self {
        Resource { id, name, amount }
    }
}
