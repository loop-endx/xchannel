use std::fmt::Display;

mod frame;
pub mod tcp;

const _MODBUS_MAX_PDU_LEN: usize = 253;

type Quantity = u16;
type Address = u16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request<'a> {
    ReadCoils(Address, Quantity),
    ReadDiscreteInputs(Address, Quantity),
    ReadHoldingRegisters(Address, Quantity),
    ReadInputRegisters(Address, Quantity),
    WriteSingleCoil(Address, bool),
    WriteSingleRegister(Address, u16),
    WriteMultipleCoils(Address, &'a [bool]),
    WriteMultipleRegisters(Address, &'a [u16]),
}

pub enum Response {
    ReadCoils(Vec<bool>),
    ReadDiscreteInputs(Vec<bool>),
    WriteSingleCoil(Address, bool),
    WriteMultipleCoils(Address, Quantity),
    ReadInputRegisters(Vec<u16>),
    ReadHoldingRegisters(Vec<u16>),
    WriteSingleRegister(Address, u16),
    WriteMultipleRegisters(Address, Quantity),
    ExceptionResponse(u8, Exception),
}

#[derive(Debug)]
pub enum Exception {
    IllegalFunction = 0x01,
    IllegalDataAddress = 0x02,
    IllegalDataValue = 0x03,
    ServerDeviceFailure = 0x04,
    Acknowledge = 0x05,
    ServerDeviceBusy = 0x06,
    MemoryParityError = 0x08,
    GateWayPathUnavailable = 0x0A,
    GatewayTargetDevice = 0x0B,
}

impl Exception {
    pub fn description(&self) -> &str {
        use Exception::*;

        match *self {
            IllegalFunction => "Illegal function",
            IllegalDataAddress => "Illegal data address",
            IllegalDataValue => "Illegal data value",
            ServerDeviceFailure => "Server device failure",
            Acknowledge => "Acknowledge",
            ServerDeviceBusy => "Server device busy",
            MemoryParityError => "Memory parity error",
            GateWayPathUnavailable => "Gate way path unavailable",
            GatewayTargetDevice => "Gateway target device failed to respond",
        }
    }
}

impl<'a> Request<'a> {
    fn code(&self) -> u8 {
        use Request::*;
        match self {
            ReadCoils(_, _) => 0x01,
            ReadDiscreteInputs(_, _) => 0x02,
            ReadHoldingRegisters(_, _) => 0x03,
            ReadInputRegisters(_, _) => 0x04,
            WriteSingleCoil(_, _) => 0x05,
            WriteSingleRegister(_, _) => 0x06,
            WriteMultipleCoils(_, _) => 0x0F,
            WriteMultipleRegisters(_, _) => 0x10,
        }
    }

    fn req_size(&self) -> usize {
        use Request::*;
        match self {
            ReadCoils(_, _)
            | ReadDiscreteInputs(_, _)
            | ReadHoldingRegisters(_, _)
            | ReadInputRegisters(_, _)
            | WriteSingleCoil(_, _)
            | WriteSingleRegister(_, _) => 5,
            WriteMultipleCoils(_, ref data) => 6 + data.len(),
            WriteMultipleRegisters(_, ref data) => 6 + data.len(),
        }
    }
}

//#[derive(Clone)]
//pub struct RequestPdu<'a>(pub Request<'a>);
//pub struct ResponsePdu(pub Result<Response, io::Error>);

//impl<'a> From<Request<'a>> for RequestPdu<'a> {
//fn from(value: Request<'a>) -> Self {
//RequestPdu(value)
//}
//}

//impl Error for ExceptionResponse {
//fn description(&self) -> &str {
//self.exception.description()
//}
//}

//impl Display for ExceptionResponse {
//fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//write!(f, "Modbus function {}: {}", self.function, self.exception)
//}
//}

impl Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
