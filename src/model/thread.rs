#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum ThreadStatus {
    Locked,
    Unlocked,
}

#[derive(Clone, PartialEq)]
pub struct Thread {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub status: ThreadStatus,
    pub sticky: bool,

    pub forum_id: i64,
    pub creator_id: i64,
}
