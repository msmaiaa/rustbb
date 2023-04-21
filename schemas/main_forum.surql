DEFINE TABLE main_forum SCHEMALESS;

DEFINE FIELD title ON main_forum TYPE string ASSERT $value != NONE;
DEFINE FIELD created_at ON main_forum TYPE datetime VALUE time::now();