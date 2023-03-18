use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub creator_id: i32,
    pub created_at: NaiveDateTime,
}
