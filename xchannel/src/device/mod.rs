mod tag;

pub enum DeviceType {
    Modbus(ModbusType),
    Siemens(SiemensType),
}

pub enum ModbusType {
    RTU,
    TCP,
}

pub enum SiemensType {
    S7_200,
    S7_200Smart,
    S7_300,
    S7_400,
    S7_1200,
    S7_1500,
}
