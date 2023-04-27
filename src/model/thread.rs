use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Thread {
    pub id: Thing,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub sticky: bool,
    pub locked: bool,
    pub forum: Thing,
    pub created_by: Thing,
    pub created_at: DateTime<Utc>,
}
