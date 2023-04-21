DEFINE TABLE forum SCHEMALESS;

DEFINE FIELD title ON forum TYPE string ASSERT $value != NONE;
DEFINE FIELD slug ON forum TYPE string ASSERT $value != NONE;
DEFINE FIELD content ON forum TYPE string ASSERT $value != NONE;
DEFINE FIELD sticky ON forum TYPE bool VALUE false;
DEFINE FIELD locked ON forum TYPE bool VALUE false;
DEFINE FIELD category ON forum TYPE record(category) ASSERT $value != NONE;
DEFINE FIELD created_by ON forum TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD created_at ON forum TYPE datetime VALUE time::now();