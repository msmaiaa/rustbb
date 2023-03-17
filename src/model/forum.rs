#[derive(Clone)]
pub struct Forum {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub category_id: i64,
}
