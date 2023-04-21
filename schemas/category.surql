DEFINE TABLE category SCHEMALESS;

DEFINE FIELD title ON category TYPE string ASSERT $value != NONE;
DEFINE FIELD description ON category TYPE string ASSERT $value != NONE;
DEFINE FIELD forums ON category TYPE array;
DEFINE FIELD created_by ON category TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD created_at ON forum_user TYPE datetime VALUE time::now();
