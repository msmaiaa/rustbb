DEFINE TABLE permission SCHEMALESS;

DEFINE FIELD tag ON permission TYPE string ASSERT $value != NONE;
DEFINE FIELD label ON permission TYPE string ASSERT $value != NONE;

DEFINE INDEX tag_idx ON TABLE permission COLUMNS tag UNIQUE;