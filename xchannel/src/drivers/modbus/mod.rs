pub mod client;
pub mod protocol;

pub mod modbus_tcp;

use crate::error::*;

use crate::module::driver::Tag;
use crate::module::value::Value;

#[derive(PartialEq, Debug)]
pub enum Area {
    Coil,
    DiscreteInput,
    InputRegister,
    HoldingRegister,
}

impl TryFrom<&str> for Area {
    type Error = XError;

    fn try_from(value: &str) -> XResult<Self> {
        use Area::*;

        match value {
            "0" => Ok(Coil),
            "1" => Ok(DiscreteInput),
            "3" => Ok(InputRegister),
            "4" => Ok(HoldingRegister),
            _ => Err(XError::new(XErrorKind::TagError, "invalid area code")),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Address {
    slave: u8,
    area: Area,
    address: u16,  // 0x0000 - 0xFFFF
    quantity: u16, // 0x0001 - 0x7D00
    bit: u8,       // 0x00 - 0x0f
    length: u16,
}

impl Address {
    fn to(
        tag: &Tag,
        slave: u8,
        area: Area,
        address: u32,
        str_address: Vec<&str>,
    ) -> XResult<Address> {
        use Value::*;
        match tag.value {
            BIT(_) | BOOL(_) => match area {
                Area::Coil | Area::DiscreteInput => Ok(Address {
                    slave,
                    area,
                    address: (address - 1) as u16,
                    quantity: 1,
                    bit: 0,
                    length: 0,
                }),
                Area::InputRegister | Area::HoldingRegister => {
                    if str_address.len() != 3 {
                        return Err(XError::new(
                            XErrorKind::TagError,
                            "address must be in the format: <slave>.<address>.<bit>",
                        ));
                    }
                    let bit = str_address[2]
                        .get(0..1)
                        .ok_or(XError::new(
                            XErrorKind::TagError,
                            "address must be in the format: <slave>.<address>.<bit>",
                        ))?
                        .parse::<u8>()
                        .map_err(|_| XError::new(XErrorKind::TagError, "need bit offset"))?;

                    Ok(Address {
                        slave,
                        area,
                        address: (address - 1) as u16,
                        quantity: 1,
                        bit,
                        length: 0,
                    })
                }
            },
            UINT16(_) | INT16(_) => match area {
                Area::Coil | Area::DiscreteInput => Err(XError::new(
                    XErrorKind::TagError,
                    "unsupport INT16/UINT16/WORD for Coil/DiscreteInput",
                )),
                Area::HoldingRegister | Area::InputRegister => Ok(Address {
                    slave,
                    area,
                    address: (address - 1) as u16,
                    quantity: 1,
                    bit: 0,
                    length: 0,
                }),
            },
            UINT32(_) | INT32(_) | FLOAT(_) => match area {
                Area::Coil | Area::DiscreteInput => Err(XError::new(
                    XErrorKind::TagError,
                    "unsupport INT32/UINT32/FLOAT/DWORD for Coil/DiscreteInput",
                )),
                Area::HoldingRegister | Area::InputRegister => Ok(Address {
                    slave,
                    area,
                    address: (address - 1) as u16,
                    quantity: 2,
                    bit: 0,
                    length: 0,
                }),
            },
            UINT64(_) | INT64(_) | DOUBLE(_) => match area {
                Area::Coil | Area::DiscreteInput => Err(XError::new(
                    XErrorKind::TagError,
                    "unsupport INT64/UINT64/DOUBLE/LWORD for Coil/DiscreteInput",
                )),
                Area::HoldingRegister | Area::InputRegister => Ok(Address {
                    slave,
                    area,
                    address: (address - 1) as u16,
                    quantity: 4,
                    bit: 0,
                    length: 0,
                }),
            },
            STRING { .. } => {
                if area == Area::Coil || area == Area::DiscreteInput {
                    return Err(XError::new(
                        XErrorKind::TagError,
                        "unsupport STRING for Coil/DiscreteInput",
                    ));
                }
                if str_address.len() != 3 {
                    return Err(XError::new(
                        XErrorKind::TagError,
                        "address must be in the format: <slave>.<address>.<length><H/L>",
                    ));
                }
                let length = str_address[2]
                    .get(0..)
                    .ok_or(XError::new(
                        XErrorKind::TagError,
                        "address must be in the format: <slave>.<address>.<length><H/L>",
                    ))?
                    .parse::<u16>()
                    .map_err(|_| XError::new(XErrorKind::TagError, "need string length"))?;

                Ok(Address {
                    slave,
                    area,
                    address: (address - 1) as u16,
                    quantity: length / 2,
                    bit: 0,
                    length,
                })
            }
            _ => Err(XError::new(
                XErrorKind::TagError,
                "invalid value type for Modbus",
            )),
        }
    }
}

impl TryFrom<&Tag> for Address {
    type Error = XError;

    // Coils
    // 1.000001 - 1.065536 decimal, start with 1
    // 1.H000001 - 1.H010000 hex, start with 1

    fn try_from(tag: &Tag) -> XResult<Self> {
        if !tag.address.is_ascii() {
            return Err(XError::new(XErrorKind::TagError, "address must be ASCII"));
        }

        let address: Vec<&str> = tag.address.split('.').collect();
        if address.len() != 2 && address.len() != 3 {
            return Err(XError::new(
                XErrorKind::TagError,
                "address must be in the format: <slave>.<address>.<length/bit><H/L>",
            ));
        }

        let slave = address[0]
            .parse::<u8>()
            .map_err(|_| XError::new(XErrorKind::TagError, "invalid slave id"))?;

        let info = if let Some("H") = address[1].get(0..1) {
            let area = address[1]
                .get(1..2)
                .ok_or(XError::new(
                    XErrorKind::TagError,
                    "address must be in the format: <slave>.<address>.<length/bit><H/L>",
                ))?
                .try_into()?;

            let reg_address = u32::from_str_radix(address[1].get(2..).unwrap(), 16)
                .map_err(|_| XError::new(XErrorKind::TagError, "invalid hex address"))?;

            (area, reg_address)
        } else {
            let area = address[1]
                .get(0..1)
                .ok_or(XError::new(
                    XErrorKind::TagError,
                    "address must be in the format: <slave>.<address>.<length/bit><H/L>",
                ))?
                .try_into()?;

            let reg_address = address[1]
                .get(1..)
                .ok_or(XError::new(
                    XErrorKind::TagError,
                    "address must be in the format: <slave>.<address>.<length/bit><H/L>",
                ))?
                .parse::<u32>()
                .map_err(|_| XError::new(XErrorKind::TagError, "invalid address"))?;

            (area, reg_address)
        };

        if info.1 > 0 && info.1 <= 0x10000 {
            Address::to(tag, slave, info.0, info.1, address)
        } else {
            Err(XError::new(
                XErrorKind::TagError,
                "address must be in the range: 1 - 65536",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, Area};
    use crate::error::*;
    use crate::module::driver::Tag;
    use crate::module::value::DataType::*;
    use crate::module::value::*;

    fn tag_check(dtype: DataType, str_address: &str, is_ok: bool, check_address: Option<Address>) {
        let tag = &Tag {
            name: "test".to_string(),
            value: dtype.default_value(),
            dtype,
            address: str_address.to_string(),
        };

        let address: XResult<Address> = tag.try_into();
        if is_ok {
            assert!(address.is_ok());
            assert_eq!(address.unwrap(), check_address.unwrap());
        } else {
            assert!(address.is_err());
        }
    }

    #[test]
    fn tag_parse_error() {
        tag_check(BIT, "1.00", false, None);
        tag_check(BIT, "1.065537", false, None);
        tag_check(BIT, "1.265537", false, None);
        tag_check(BYTE, "1.01", false, None);
        tag_check(BYTE, "1.11", false, None);
    }

    #[test]
    fn tag_parse_hex() {
        let address = Address {
            slave: 1,
            area: Area::Coil,
            address: 15,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(BIT, "1.H010", true, Some(address));
    }

    #[test]
    fn tag_parse_coil() {
        let address = Address {
            slave: 1,
            area: Area::Coil,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(BIT, "1.01", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::Coil,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(BOOL, "1.01", true, Some(address));
    }

    #[test]
    fn tag_parse_input() {
        let address = Address {
            slave: 1,
            area: Area::DiscreteInput,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(BIT, "1.11", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::DiscreteInput,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(BOOL, "1.11", true, Some(address));
    }

    #[test]
    fn tag_parse_input_reg() {
        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(WORD, "1.31", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(INT, "1.31", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(WORD, "1.31", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(DINT, "1.31", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(UDINT, "1.31", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(FLOAT, "1.31", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(DWORD, "1.31", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(LINT, "1.31", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(ULINT, "1.31", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(DOUBLE, "1.31", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(LWORD, "1.31", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 5,
            bit: 0,
            length: 10,
        };
        tag_check(STRING, "1.31.10", true, Some(address));

        let _address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 5,
            bit: 0,
            length: 10,
        };
    }

    #[test]
    fn tag_parse_hold_reg() {
        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(WORD, "1.41", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(INT, "1.41", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(WORD, "1.41", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(DINT, "1.41", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(UDINT, "1.41", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(FLOAT, "1.41", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(DWORD, "1.41", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(LINT, "1.41", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(UDINT, "1.41", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(DOUBLE, "1.41", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(LWORD, "1.41", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 5,
            bit: 0,
            length: 10,
        };
        tag_check(STRING, "1.41.10", true, Some(address));

        let _address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 5,
            bit: 0,
            length: 10,
        };
    }
}
