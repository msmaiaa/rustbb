DEFINE TABLE post SCHEMALESS;

DEFINE FIELD content ON post TYPE string ASSERT $value != NONE;
DEFINE FIELD thread ON post TYPE record(thread) ASSERT $value != NONE;
DEFINE FIELD created_by ON post TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD created_at ON post TYPE datetime VALUE time::now();