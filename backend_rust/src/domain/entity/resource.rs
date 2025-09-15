#[derive(Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct ResourceId(pub i32);

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Resource {
    pub id: ResourceId,
    pub name: String,
    pub amount: i32,
}

impl Resource {
    pub fn new(id: i32, name: String, amount: i32) -> Self {
        let id = ResourceId(id);
        Resource { id, name, amount }
    }
}
