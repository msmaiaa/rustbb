#[derive(Clone)]
pub struct Forum {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub category_id: i32,
}
