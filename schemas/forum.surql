DEFINE TABLE forum SCHEMALESS;

DEFINE FIELD title ON forum TYPE string ASSERT $value != NONE;
DEFINE FIELD description ON forum TYPE string;
DEFINE FIELD slug ON forum TYPE string ASSERT $value != NONE;
DEFINE FIELD category ON forum TYPE record(category) ASSERT $value != NONE;
DEFINE FIELD created_at ON forum TYPE datetime VALUE time::now();