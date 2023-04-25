DEFINE TABLE permission SCHEMALESS;

DEFINE FIELD description ON permission TYPE string ASSERT $value != NONE;
DEFINE FIELD value_kind ON permission TYPE string ASSERT $value IN ['Boolean', 'Integer', 'Float'];