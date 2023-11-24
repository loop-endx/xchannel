use std::io::{Cursor, Error, ErrorKind};

use byteorder::{BigEndian, ReadBytesExt as _};
use bytes::{BufMut, Bytes, BytesMut};

use super::{Exception, ExceptionResponse, Request, RequestPdu, Response, ResponsePdu};

fn bool_to_coil(state: bool) -> u16 {
    if state {
        0xFF00
    } else {
        0x0000
    }
}

fn coil_to_bool(coil: u16) -> bool {
    coil & 0xFF00 > 0
}

fn pack_coils(coils: &[bool]) -> Vec<u8> {
    let mut res = vec![0; (coils.len() + 7) / 8];

    for (i, b) in coils.iter().enumerate() {
        let v = u8::from(*b);
        res[i / 8] |= v << (i % 8);
    }

    res
}

fn unpack_coils(bytes: &[u8], count: u16) -> Vec<bool> {
    let mut res = Vec::with_capacity(count as usize);
    for i in 0usize..count.into() {
        res.push((bytes[i / 8] >> (i % 8)) & 0b1 > 0);
    }
    res
}

impl<'a> From<Request<'a>> for Bytes {
    fn from(f: Request<'a>) -> Self {
        use Request::*;

        let mut data = BytesMut::with_capacity(f.req_size());
        data.put_u8(f.code());
        match f {
            ReadCoils(address, quantity)
            | ReadDiscreteInputs(address, quantity)
            | ReadInputRegisters(address, quantity)
            | ReadHoldingRegisters(address, quantity) => {
                data.put_u16(address);
                data.put_u16(quantity);
            }
            WriteSingleCoil(address, state) => {
                data.put_u16(address);
                data.put_u16(bool_to_coil(state));
            }
            WriteMultipleCoils(address, coils) => {
                data.put_u16(address);
                data.put_u16(coils.len() as u16);
                let packed_coils = pack_coils(coils);
                data.put_u8(packed_coils.len() as u8);
                for b in packed_coils {
                    data.put_u8(b);
                }
            }
            WriteSingleRegister(address, word) => {
                data.put_u16(address);
                data.put_u16(word);
            }
            WriteMultipleRegisters(address, words) => {
                data.put_u16(address);
                data.put_u16(words.len() as u16);
                data.put_u8(words.len() as u8 * 2);
                for v in words {
                    data.put_u16(*v);
                }
            }
        }

        data.freeze()
    }
}

impl TryFrom<Bytes> for Response {
    type Error = Error;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        use Response::*;

        let mut rdr = Cursor::new(&bytes);
        let function = rdr.read_u8()?;

        let rsp = match function {
            0x01 => {
                let byte_count = rdr.read_u8()?;
                let x = &bytes[2..];

                let quantity = u16::from(byte_count) * 8;
                ReadCoils(unpack_coils(x, quantity))
            }
            0x02 => {
                let byte_count = rdr.read_u8()?;
                let x = &bytes[2..];

                let quantity = u16::from(byte_count) * 8;
                ReadDiscreteInputs(unpack_coils(x, quantity))
            }
            0x03 => {
                let byte_count = rdr.read_u8()?;
                let quantity = byte_count / 2;
                let mut data = Vec::with_capacity(quantity as usize);

                for _ in 0..quantity {
                    data.push(rdr.read_u16::<BigEndian>()?);
                }
                ReadHoldingRegisters(data)
            }
            0x04 => {
                let byte_count = rdr.read_u8()?;
                let quantity = byte_count / 2;
                let mut data = Vec::with_capacity(quantity as usize);

                for _ in 0..quantity {
                    data.push(rdr.read_u16::<BigEndian>()?);
                }
                ReadInputRegisters(data)
            }
            0x05 => {
                let address = rdr.read_u16::<BigEndian>()?;
                let value = rdr.read_u16::<BigEndian>()?;
                WriteSingleCoil(address, coil_to_bool(value))
            }
            0x06 => {
                let address = rdr.read_u16::<BigEndian>()?;
                let value = rdr.read_u16::<BigEndian>()?;
                WriteSingleRegister(address, value)
            }
            0x0F => {
                let address = rdr.read_u16::<BigEndian>()?;
                let quantity = rdr.read_u16::<BigEndian>()?;
                WriteMultipleCoils(address, quantity)
            }
            0x10 => {
                let address = rdr.read_u16::<BigEndian>()?;
                let quantity = rdr.read_u16::<BigEndian>()?;
                WriteMultipleRegisters(address, quantity)
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid function code: {}", function),
                ))
            }
        };

        Ok(rsp)
    }
}

impl TryFrom<Bytes> for ExceptionResponse {
    type Error = Error;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        let mut rdr = Cursor::new(&bytes);
        let err_code = rdr.read_u8()?;

        if err_code < 0x80 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid exception code: {}", err_code),
            ));
        }

        let function = err_code - 0x80;
        let exception = Exception::try_from(rdr.read_u8()?)?;
        Ok(ExceptionResponse {
            function,
            exception,
        })
    }
}

impl TryFrom<u8> for Exception {
    type Error = Error;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        use Exception::*;

        let ex = match code {
            0x01 => IllegalFunction,
            0x02 => IllegalDataAddress,
            0x03 => IllegalDataValue,
            0x04 => ServerDeviceFailure,
            0x05 => Acknowledge,
            0x06 => ServerDeviceBusy,
            0x08 => MemoryParityError,
            0x0A => GateWayPathUnavailable,
            0x0B => GatewayTargetDevice,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid exception code: {}", code),
                ))
            }
        };

        Ok(ex)
    }
}

impl TryFrom<Bytes> for ResponsePdu {
    type Error = Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let code = Cursor::new(&value).read_u8()?;
        if code < 0x80 {
            Ok(Response::try_from(value)?.into())
        } else {
            Ok(ExceptionResponse::try_from(value)?.into())
        }
    }
}

impl<'a> TryFrom<RequestPdu<'a>> for Bytes {
    type Error = Error;

    fn try_from(value: RequestPdu<'a>) -> Result<Self, Self::Error> {
        Ok(value.0.into())
    }
}

impl From<Response> for ResponsePdu {
    fn from(value: Response) -> Self {
        ResponsePdu(Ok(value))
    }
}

impl From<ExceptionResponse> for ResponsePdu {
    fn from(value: ExceptionResponse) -> Self {
        ResponsePdu(Err(value))
    }
}
