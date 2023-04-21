DEFINE TABLE permission SCHEMALESS;

DEFINE FIELD label ON permission TYPE string ASSERT $value != NONE;
