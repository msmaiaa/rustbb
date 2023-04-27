DEFINE TABLE user SCHEMALESS;

DEFINE FIELD username ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD email ON user TYPE string ASSERT $value != NONE AND is::email($value);
DEFINE FIELD avatar_url ON user TYPE string;
DEFINE FIELD user_group ON user ASSERT $value != NONE;
DEFINE FIELD password ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD created_at ON user TYPE datetime VALUE time::now();

DEFINE INDEX username ON TABLE user COLUMNS username UNIQUE;
DEFINE INDEX email ON TABLE user COLUMNS email UNIQUE;