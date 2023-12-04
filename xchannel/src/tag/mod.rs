pub struct Tag {
    pub name: String,
    pub value: Value,
    pub address: String,
    pub description: Option<String>,
}

pub enum Value {
    Base(BaseValue),
    Array(Vec<BaseValue>),
    Struct(Vec<BaseValue>),
    Series(Vec<BaseValue>),
}

pub enum BaseValue {
    BIT(u8),
    BOOL(bool),
    UINT8(u8),
    INT8(i8),
    UINT16(u16),
    INT16(i16),
    WORD(u16),
    UINT32(u32),
    INT32(i32),
    FLOAT(f32),
    DWORD(u32),
    UINT64(u64),
    INT64(i64),
    DOUBLE(f64),
    LWORD(u64),
    ERROR(u64),
    STRING(u16, Option<String>),
    BYTES(u16, Option<Vec<u8>>),
}
