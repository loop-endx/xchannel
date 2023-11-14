pub struct Tag {
    name: String,
    address: String,
    value: Value,
    description: String,
}

pub enum Value {
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
    STRING(String),
    BYTES(Vec<u8>),
}
