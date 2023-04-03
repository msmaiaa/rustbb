use lazy_static::lazy_static;

pub type Id = &'static str;
pub type Label = &'static str;

#[derive(Clone, Debug)]
pub enum ValueType {
    Boolean,
    Integer,
}
pub type PermissionEntry = (Id, Label, ValueType);

#[derive(Clone, Debug)]
pub struct StaticPermissionEntries(pub Vec<PermissionEntry>);

lazy_static! {
    pub static ref PERMISSION_ENTRIES: StaticPermissionEntries = StaticPermissionEntries(vec![
        ("thread.create", "Create threads", ValueType::Boolean),
        ("thread.edit", "Edit own threads", ValueType::Boolean),
        ("thread.delete", "Delete own threads", ValueType::Boolean),
        (
            "thread.edit_any",
            "Edit threads from any user",
            ValueType::Boolean,
        ),
        (
            "thread.delete_any",
            "Delete threads from any user",
            ValueType::Boolean,
        ),
        ("thread.move_any", "Move threads", ValueType::Boolean),
        ("thread.approve", "Approve threads", ValueType::Boolean),
        ("thread.delete", "Delete threads", ValueType::Boolean),
        (
            "thread.subscribe",
            "Subscribe to threads",
            ValueType::Boolean,
        ),
        (
            "thread.unsubscribe",
            "Unsubscribe from threads",
            ValueType::Boolean,
        ),
        ("category.create", "Create categories", ValueType::Boolean),
        ("category.edit", "Edit categories", ValueType::Boolean),
        ("category.delete", "Delete categories", ValueType::Boolean),
        ("forum.create", "Create a forum", ValueType::Boolean),
        ("forum.edit", "Edit forums", ValueType::Boolean),
        ("forum.delete", "Delete forums", ValueType::Boolean),
        ("post.create", "Create posts", ValueType::Boolean),
        ("post.edit", "Edit own post", ValueType::Boolean),
        (
            "post.edit_any",
            "Edit posts from any user",
            ValueType::Boolean,
        ),
        ("post.delete", "Delete own posts", ValueType::Boolean),
        (
            "post.delete_any",
            "Delete posts from any user",
            ValueType::Boolean,
        ),
    ]);
}
