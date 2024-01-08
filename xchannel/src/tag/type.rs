use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    VT(ValueType),
    WORD,
    DWORD,
    LWORD,
    Real,
    LReal,
}

impl DataType {
    pub fn init(&self) -> Value {
        use DataType::*;
        use ValueType::*;
        match self {
            DataType::VT(vt) => match vt {
                BIT => Value::BIT(0),
                BOOL => Value::BOOL(false),
                UINT8 => Value::UINT8(0),
                INT8 => Value::INT8(0),
                UINT16 => Value::UINT16(0),
                INT16 => Value::INT16(0),
                UINT32 => Value::UINT32(0),
                INT32 => Value::INT32(0),
                FLOAT => Value::FLOAT(0.0),
                UINT64 => Value::UINT64(0),
                INT64 => Value::INT64(0),
                DOUBLE => Value::DOUBLE(0.0),
                STRING => Value::STRING {
                    length: 128,
                    str: None,
                },
            },
            WORD => Value::UINT16(0),
            DWORD => Value::UINT32(0),
            LWORD => Value::UINT64(0),
            Real => Value::FLOAT(0.0),
            LReal => Value::DOUBLE(0.0),
        }
    }
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
    STRING { length: u16, str: Option<String> },
}

impl Value {
    pub fn r#type(&self) -> ValueType {
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
