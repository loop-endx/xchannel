pub mod modbus_rtu;
pub mod modbus_tcp;

pub mod client;
pub mod protocol;

pub enum Area {
    Coil,
    DiscreteInput,
    InputRegister,
    HoldingRegister,
}
pub struct Address {
    slave: u8,
    area: Area,
    address: u16,  // 0x0000 - 0xFFFF
    qunatity: u16, // 0x0001 - 0x7D00
}
