DEFINE TABLE thread SCHEMALESS;

DEFINE FIELD title ON thread TYPE string ASSERT $value != NONE;
DEFINE FIELD slug ON thread TYPE string ASSERT $value != NONE;
DEFINE FIELD content ON thread TYPE string ASSERT $value != NONE;
DEFINE FIELD sticky ON thread TYPE bool VALUE $value OR false;
DEFINE FIELD locked ON thread TYPE bool VALUE $value OR false;
DEFINE FIELD forum ON thread TYPE record(forum) ASSERT $value != NONE;
DEFINE FIELD created_by ON thread TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD created_at ON thread TYPE datetime VALUE time::now();