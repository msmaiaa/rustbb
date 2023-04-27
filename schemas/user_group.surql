DEFINE TABLE user_group SCHEMALESS;

DEFINE FIELD name ON user_group TYPE string ASSERT $value != NONE;
DEFINE FIELD user_title ON user_group TYPE string ASSERT $value != NONE;
DEFINE FIELD description ON user_group TYPE string;
DEFINE FIELD permissions ON user_group TYPE array;

DEFINE INDEX name_idx ON TABLE user_group COLUMNS name UNIQUE;