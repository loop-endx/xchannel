-- device table
DEFINE TABLE device SCHEMALESS;

DEFINE FIELD name ON TABLE device TYPE string;
DEFINE FIELD driver ON TABLE device TYPE string;
DEFINE INDEX nameIndex ON TABLE device COLUMNS name UNIQUE;
