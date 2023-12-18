pub mod client;
pub mod protocol;

pub mod modbus_tcp;

use crate::error::*;

use crate::module::driver::tag::Tag;
use crate::module::tag::BaseValue;

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
            _ => Err(TagError::new(
                TagErrorKind::InvalidAddress,
                "invalid area code",
            )),
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
        if !tag.value.is_base() {
            return Err(TagError::new(
                TagErrorKind::UnsupportType,
                "invalid value type for Modbus",
            ));
        }

        let v = &tag.value;
        let base_value: &BaseValue = v.try_into()?;

        match base_value {
            BaseValue::BIT(_) | BaseValue::BOOL(_) => match area {
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
                        return Err(TagError::new(
                            TagErrorKind::InvalidAddress,
                            "address must be in the format: <slave>.<address>.<bit>",
                        ));
                    }
                    let bit = str_address[2]
                        .get(0..1)
                        .ok_or(TagError::new(
                            TagErrorKind::InvalidAddress,
                            "address must be in the format: <slave>.<address>.<bit>",
                        ))?
                        .parse::<u8>()
                        .map_err(|_| {
                            TagError::new(TagErrorKind::InvalidAddress, "need bit offset")
                        })?;

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
            BaseValue::UINT16(_) | BaseValue::INT16(_) | BaseValue::WORD(_) => match area {
                Area::Coil | Area::DiscreteInput => Err(TagError::new(
                    TagErrorKind::UnsupportType,
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
            BaseValue::UINT32(_)
            | BaseValue::INT32(_)
            | BaseValue::FLOAT(_)
            | BaseValue::DWORD(_) => match area {
                Area::Coil | Area::DiscreteInput => Err(TagError::new(
                    TagErrorKind::UnsupportType,
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
            BaseValue::UINT64(_)
            | BaseValue::INT64(_)
            | BaseValue::DOUBLE(_)
            | BaseValue::LWORD(_) => match area {
                Area::Coil | Area::DiscreteInput => Err(TagError::new(
                    TagErrorKind::UnsupportType,
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
            BaseValue::STRING { .. } => {
                if area == Area::Coil || area == Area::DiscreteInput {
                    return Err(TagError::new(
                        TagErrorKind::UnsupportType,
                        "unsupport STRING for Coil/DiscreteInput",
                    ));
                }
                if str_address.len() != 3 {
                    return Err(TagError::new(
                        TagErrorKind::InvalidAddress,
                        "address must be in the format: <slave>.<address>.<length><H/L>",
                    ));
                }
                let length = str_address[2]
                    .get(0..)
                    .ok_or(TagError::new(
                        TagErrorKind::InvalidAddress,
                        "address must be in the format: <slave>.<address>.<length><H/L>",
                    ))?
                    .parse::<u16>()
                    .map_err(|_| {
                        TagError::new(TagErrorKind::InvalidAddress, "need string length")
                    })?;

                Ok(Address {
                    slave,
                    area,
                    address: (address - 1) as u16,
                    quantity: length / 2,
                    bit: 0,
                    length,
                })
            }
            BaseValue::BYTES { .. } => {
                if area == Area::Coil || area == Area::DiscreteInput {
                    return Err(TagError::new(
                        TagErrorKind::UnsupportType,
                        "unsupport BYTES for Coil/DiscreteInput",
                    ));
                }
                if str_address.len() != 3 {
                    return Err(TagError::new(
                        TagErrorKind::InvalidAddress,
                        "address must be in the format: <slave>.<address>.<length>",
                    ));
                }

                let length = str_address[2]
                    .get(0..)
                    .ok_or(TagError::new(
                        TagErrorKind::InvalidAddress,
                        "address must be in the format: <slave>.<address>.<length>",
                    ))?
                    .parse::<u16>()
                    .map_err(|_| {
                        TagError::new(TagErrorKind::InvalidAddress, "need bytes length")
                    })?;

                Ok(Address {
                    slave,
                    area,
                    address: (address - 1) as u16,
                    quantity: length / 2,
                    bit: 0,
                    length,
                })
            }
            _ => Err(TagError::new(
                TagErrorKind::UnsupportType,
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
            return Err(TagError::new(
                TagErrorKind::InvalidAddress,
                "address must be ASCII",
            ));
        }

        let address: Vec<&str> = tag.address.split('.').collect();
        if address.len() != 2 && address.len() != 3 {
            return Err(TagError::new(
                TagErrorKind::InvalidAddress,
                "address must be in the format: <slave>.<address>.<length/bit><H/L>",
            ));
        }

        let slave = address[0]
            .parse::<u8>()
            .map_err(|_| TagError::new(TagErrorKind::InvalidAddress, "invalid slave id"))?;

        let info = if let Some("H") = address[1].get(0..1) {
            let area = address[1]
                .get(1..2)
                .ok_or(TagError::new(
                    TagErrorKind::InvalidAddress,
                    "address must be in the format: <slave>.<address>.<length/bit><H/L>",
                ))?
                .try_into()?;

            let reg_address = u32::from_str_radix(address[1].get(2..).unwrap(), 16)
                .map_err(|_| TagError::new(TagErrorKind::InvalidAddress, "invalid hex address"))?;

            (area, reg_address)
        } else {
            let area = address[1]
                .get(0..1)
                .ok_or(TagError::new(
                    TagErrorKind::InvalidAddress,
                    "address must be in the format: <slave>.<address>.<length/bit><H/L>",
                ))?
                .try_into()?;

            let reg_address = address[1]
                .get(1..)
                .ok_or(TagError::new(
                    TagErrorKind::InvalidAddress,
                    "address must be in the format: <slave>.<address>.<length/bit><H/L>",
                ))?
                .parse::<u32>()
                .map_err(|_| TagError::new(TagErrorKind::InvalidAddress, "invalid address"))?;

            (area, reg_address)
        };

        if info.1 > 0 && info.1 <= 0x10000 {
            Address::to(tag, slave, info.0, info.1, address)
        } else {
            Err(TagError::new(
                TagErrorKind::InvalidAddress,
                "address must be in the range: 1 - 65536",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, Area};
    use crate::error::*;
    use crate::module::driver::tag::Tag;
    use crate::module::tag::{BaseValue, Value};

    fn tag_check(value: Value, str_address: &str, is_ok: bool, check_address: Option<Address>) {
        let tag = &Tag {
            name: "test".to_string(),
            value,
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
        tag_check(Value::Base(BaseValue::BIT(0)), "1.00", false, None);
        tag_check(Value::Base(BaseValue::BIT(0)), "1.065537", false, None);
        tag_check(Value::Base(BaseValue::BIT(0)), "1.265537", false, None);
        tag_check(Value::Base(BaseValue::UINT8(0)), "1.01", false, None);
        tag_check(Value::Base(BaseValue::UINT8(0)), "1.11", false, None);
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
        tag_check(
            Value::Base(BaseValue::BIT(0)),
            "1.H010",
            true,
            Some(address),
        );
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
        tag_check(Value::Base(BaseValue::BIT(0)), "1.01", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::Coil,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::BOOL(false)),
            "1.01",
            true,
            Some(address),
        );
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
        tag_check(Value::Base(BaseValue::BIT(0)), "1.11", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::DiscreteInput,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::BOOL(false)),
            "1.11",
            true,
            Some(address),
        );
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
        tag_check(
            Value::Base(BaseValue::UINT16(0)),
            "1.31",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::INT16(0)),
            "1.31",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(Value::Base(BaseValue::WORD(0)), "1.31", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::INT32(0)),
            "1.31",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::UINT32(0)),
            "1.31",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::FLOAT(0.0)),
            "1.31",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::DWORD(0)),
            "1.31",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::INT64(0)),
            "1.31",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::UINT64(0)),
            "1.31",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::DOUBLE(0.0)),
            "1.31",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::LWORD(0)),
            "1.31",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 5,
            bit: 0,
            length: 10,
        };
        tag_check(
            Value::Base(BaseValue::STRING {
                length: 0,
                str: None,
            }),
            "1.31.10",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::InputRegister,
            address: 0,
            quantity: 5,
            bit: 0,
            length: 10,
        };
        tag_check(
            Value::Base(BaseValue::BYTES {
                length: 0,
                byte: None,
            }),
            "1.31.10",
            true,
            Some(address),
        );
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
        tag_check(
            Value::Base(BaseValue::UINT16(0)),
            "1.41",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::INT16(0)),
            "1.41",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 1,
            bit: 0,
            length: 0,
        };
        tag_check(Value::Base(BaseValue::WORD(0)), "1.41", true, Some(address));

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::INT32(0)),
            "1.41",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::UINT32(0)),
            "1.41",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::FLOAT(0.0)),
            "1.41",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 2,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::DWORD(0)),
            "1.41",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::INT64(0)),
            "1.41",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::UINT64(0)),
            "1.41",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::DOUBLE(0.0)),
            "1.41",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 4,
            bit: 0,
            length: 0,
        };
        tag_check(
            Value::Base(BaseValue::LWORD(0)),
            "1.41",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 5,
            bit: 0,
            length: 10,
        };
        tag_check(
            Value::Base(BaseValue::STRING {
                length: 0,
                str: None,
            }),
            "1.41.10",
            true,
            Some(address),
        );

        let address = Address {
            slave: 1,
            area: Area::HoldingRegister,
            address: 0,
            quantity: 5,
            bit: 0,
            length: 10,
        };
        tag_check(
            Value::Base(BaseValue::BYTES {
                length: 0,
                byte: None,
            }),
            "1.41.10",
            true,
            Some(address),
        );
    }
}
