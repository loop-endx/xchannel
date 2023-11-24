//use std::fmt;

use crate::driver::DriverInfo;

//struct ModbusRTUSetting {}
//struct ModbusTCPSetting {}

//enum DriverT {
//ModbusRTU(Box<dyn Driver<Setting = ModbusRTUSetting>>),
//ModbusTCP(Box<dyn Driver<Setting = ModbusTCPSetting>>),
//}

#[warn(dead_code)]
pub struct Drivers {
    drivers: Vec<u8>,
}

impl Drivers {
    pub fn new() -> Drivers {
        Drivers {
            drivers: vec![
                //DriverT::ModbusRTU(Box::new(crate::driver::modbus::ModbusRTU::new())),
                //DriverT::ModbusTCP(Box::new(crate::driver::modbus::ModbusTCP::new())),
            ],
        }
    }

    pub fn get_drivers(&self) -> Vec<DriverInfo> {
        //let mut drivers: Vec<DriverInfo> = Vec::new();
        //for driver in &self.drivers {
        //match driver {
        //DriverT::ModbusTCP(x) => drivers.push(x.info()),
        //DriverT::ModbusRTU(x) => drivers.push(x.info()),
        //}
        //}
        //drivers
        vec![DriverInfo {
            name: "Modbus TCP".to_string(),
            description: "Modbus TCP driver".to_string(),
        }]
    }
}

//pub enum Driver {
//Modbus(Modbus),
//Siemens(Siemens),
//}

//impl fmt::Display for Driver {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//match self {
//Driver::Modbus(x) => write!(f, "Modbus {}", x),
//Driver::Siemens(x) => write!(f, "Siemens {}", x),
//}
//}
//}

//pub enum Modbus {
//RTU,
//TCP,
//}

//impl fmt::Display for Modbus {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//match self {
//Modbus::RTU => write!(f, "RTU"),
//Modbus::TCP => write!(f, "TCP"),
//}
//}
//}

//pub enum Siemens {
//S7_200,
//S7_200Smart,
//S7_300,
//S7_400,
//S7_1200,
//S7_1500,
//}

//impl fmt::Display for Siemens {
//fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
//match self {
//Siemens::S7_200 => write!(f, "S7-200"),
//Siemens::S7_200Smart => write!(f, "S7-200 Smart"),
//Siemens::S7_300 => write!(f, "S7-300"),
//Siemens::S7_400 => write!(f, "S7-400"),
//Siemens::S7_1200 => write!(f, "S7-1200"),
//Siemens::S7_1500 => write!(f, "S7-1500"),
//}
//}
//}
