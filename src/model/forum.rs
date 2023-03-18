use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Forum {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub category_id: i32,
    pub created_at: NaiveDateTime,
}
