-- device
DEFINE TABLE device SCHEMALESS;

DEFINE FIELD name ON TABLE device TYPE string;
DEFINE FIELD driver ON TABLE device TYPE string;
DEFINE INDEX nameIndex ON TABLE device COLUMNS name UNIQUE;

-- table
DEFINE TABLE table SCHEMALESS;

DEFINE FIELD device ON TABLE table TYPE record<device>;
DEFINE FIELD name ON TABLE table TYPE string;
DEFINE FIELD description ON TABLE table TYPE option<string>;
DEFINE INDEX deviceNameIndex ON TABLE table COLUMNS device, name UNIQUE;
DEFINE INDEX deviceIndex ON TABLE table COLUMNS device;
