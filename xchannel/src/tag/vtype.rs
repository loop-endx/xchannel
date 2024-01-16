use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValueType {
    BIT,
    BOOL,
    UINT8,
    INT8,
    UINT16,
    INT16,
    UINT32,
    INT32,
    FLOAT,
    UINT64,
    INT64,
    DOUBLE,
    STRING,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    BIT(u8),
    BOOL(bool),
    UINT8(u8),
    INT8(i8),
    UINT16(u16),
    INT16(i16),
    UINT32(u32),
    INT32(i32),
    FLOAT(f32),
    UINT64(u64),
    INT64(i64),
    DOUBLE(f64),
    STRING {
        length: Option<u16>,
        str: Option<String>,
    },
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum DataType {
    BIT,
    BOOL,
    BYTE,
    CHAR,
    WCHAR,
    WORD,
    DWORD,
    LWORD,
    SINT,
    USINT,
    INT,
    UINT,
    DINT,
    UDINT,
    LINT,
    ULINT,
    Real,
    LReal,
    FLOAT,
    DOUBLE,
    STRING,
    WSTRING,
}

impl From<DataType> for ValueType {
    fn from(value: DataType) -> Self {
        match value {
            DataType::BIT => ValueType::BIT,
            DataType::BOOL => ValueType::BOOL,
            DataType::BYTE => ValueType::UINT8,
            DataType::CHAR => ValueType::UINT8,
            DataType::WCHAR => ValueType::UINT16,
            DataType::WORD => ValueType::UINT16,
            DataType::DWORD => ValueType::UINT32,
            DataType::LWORD => ValueType::UINT64,
            DataType::SINT => ValueType::INT8,
            DataType::USINT => ValueType::UINT8,
            DataType::INT => ValueType::INT16,
            DataType::UINT => ValueType::UINT16,
            DataType::DINT => ValueType::INT32,
            DataType::UDINT => ValueType::UINT32,
            DataType::LINT => ValueType::INT64,
            DataType::ULINT => ValueType::UINT64,
            DataType::Real => ValueType::FLOAT,
            DataType::LReal => ValueType::DOUBLE,
            DataType::FLOAT => ValueType::FLOAT,
            DataType::DOUBLE => ValueType::DOUBLE,
            DataType::STRING => ValueType::STRING,
            DataType::WSTRING => ValueType::STRING,
        }
    }
}

impl DataType {
    #[allow(dead_code)]
    pub fn default_value(&self) -> Value {
        match self {
            DataType::BIT => Value::BIT(0),
            DataType::BOOL => Value::BOOL(false),
            DataType::BYTE => Value::UINT8(0),
            DataType::CHAR => Value::UINT8(0),
            DataType::WCHAR => Value::UINT16(0),
            DataType::WORD => Value::UINT16(0),
            DataType::DWORD => Value::UINT32(0),
            DataType::LWORD => Value::UINT64(0),
            DataType::SINT => Value::INT8(0),
            DataType::USINT => Value::UINT8(0),
            DataType::INT => Value::INT16(0),
            DataType::UINT => Value::UINT16(0),
            DataType::DINT => Value::INT32(0),
            DataType::UDINT => Value::UINT32(0),
            DataType::LINT => Value::INT64(0),
            DataType::ULINT => Value::UINT64(0),
            DataType::Real => Value::FLOAT(0.0),
            DataType::LReal => Value::DOUBLE(0.0),
            DataType::FLOAT => Value::FLOAT(0.0),
            DataType::DOUBLE => Value::DOUBLE(0.0),
            DataType::STRING => Value::STRING {
                length: None,
                str: None,
            },
            DataType::WSTRING => Value::STRING {
                length: None,
                str: None,
            },
        }
    }
}

impl ValueType {
    pub fn default_value(&self) -> Value {
        match self {
            ValueType::BIT => Value::BIT(0),
            ValueType::BOOL => Value::BOOL(false),
            ValueType::UINT8 => Value::UINT8(0),
            ValueType::INT8 => Value::INT8(0),
            ValueType::UINT16 => Value::UINT16(0),
            ValueType::INT16 => Value::INT16(0),
            ValueType::UINT32 => Value::UINT32(0),
            ValueType::INT32 => Value::INT32(0),
            ValueType::FLOAT => Value::FLOAT(0.0),
            ValueType::UINT64 => Value::UINT64(0),
            ValueType::INT64 => Value::INT64(0),
            ValueType::DOUBLE => Value::DOUBLE(0.0),
            ValueType::STRING => Value::STRING {
                length: None,
                str: None,
            },
        }
    }
}

impl Value {
    pub fn v_type(&self) -> ValueType {
        match self {
            Value::BIT(_) => ValueType::BIT,
            Value::BOOL(_) => ValueType::BOOL,
            Value::UINT8(_) => ValueType::UINT8,
            Value::INT8(_) => ValueType::INT8,
            Value::UINT16(_) => ValueType::UINT16,
            Value::INT16(_) => ValueType::INT16,
            Value::UINT32(_) => ValueType::UINT32,
            Value::INT32(_) => ValueType::INT32,
            Value::FLOAT(_) => ValueType::FLOAT,
            Value::UINT64(_) => ValueType::UINT64,
            Value::INT64(_) => ValueType::INT64,
            Value::DOUBLE(_) => ValueType::DOUBLE,
            Value::STRING { length: _, str: _ } => ValueType::STRING,
        }
    }
}
