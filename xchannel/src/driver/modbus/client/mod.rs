use std::io::{Error, ErrorKind};

use async_trait::async_trait;

use super::protocol::{Request, Response};

mod tcp;

#[async_trait]
pub trait Client: Send {
    async fn call(&mut self, slave_id: u8, request: Request<'_>) -> Result<Response, Error>;
}

#[async_trait]
pub trait AsyncModbus: Client {
    async fn read_coils(
        &mut self,
        slave_id: u8,
        address: u16,
        quantity: u16,
    ) -> Result<Vec<bool>, Error> {
        let rsp = Self::call(self, slave_id, Request::ReadCoils(address, quantity)).await?;

        if let Response::ReadCoils(mut coils) = rsp {
            debug_assert!(coils.len() >= quantity.into());
            coils.truncate(quantity.into());
            Ok(coils)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "unexpected response"))
        }
    }

    async fn read_discrete_inputs(
        &mut self,
        slave_id: u8,
        address: u16,
        quantity: u16,
    ) -> Result<Vec<bool>, Error> {
        let rsp = Self::call(
            self,
            slave_id,
            Request::ReadDiscreteInputs(address, quantity),
        )
        .await?;

        if let Response::ReadDiscreteInputs(mut coils) = rsp {
            coils.truncate(quantity.into());
            Ok(coils)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "unexpected response"))
        }
    }

    async fn read_input_registers(
        &mut self,
        slave_id: u8,
        address: u16,
        quantity: u16,
    ) -> Result<Vec<u16>, Error> {
        let rsp = Self::call(
            self,
            slave_id,
            Request::ReadInputRegisters(address, quantity),
        )
        .await?;

        if let Response::ReadInputRegisters(rsp) = rsp {
            if rsp.len() != quantity as usize {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid response"));
            }
            Ok(rsp)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "unexpected response"))
        }
    }

    async fn read_hold_registers(
        &mut self,
        slave_id: u8,
        address: u16,
        quantity: u16,
    ) -> Result<Vec<u16>, Error> {
        let rsp = Self::call(
            self,
            slave_id,
            Request::ReadHoldingRegisters(address, quantity),
        )
        .await?;

        if let Response::ReadHoldingRegisters(rsp) = rsp {
            if rsp.len() != quantity as usize {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid response"));
            }
            Ok(rsp)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "unexpected response"))
        }
    }

    async fn write_single_coil(
        &mut self,
        slave_id: u8,
        address: u16,
        value: bool,
    ) -> Result<(), Error> {
        let rsp = Self::call(self, slave_id, Request::WriteSingleCoil(address, value)).await?;

        if let Response::WriteSingleCoil(rsp_addr, rsp_coil) = rsp {
            if rsp_addr != address || rsp_coil != value {
                return Err(Error::new(ErrorKind::InvalidData, "invalid response"));
            }
            Ok(())
        } else {
            Err(Error::new(ErrorKind::InvalidData, "unexpected response"))
        }
    }

    async fn write_single_register(
        &mut self,
        slave_id: u8,
        address: u16,
        data: u16,
    ) -> Result<(), Error> {
        let rsp = Self::call(self, slave_id, Request::WriteSingleRegister(address, data)).await?;

        if let Response::WriteSingleRegister(rsp_addr, rsp_word) = rsp {
            if rsp_addr != address || rsp_word != data {
                return Err(Error::new(ErrorKind::InvalidData, "invalid response"));
            }
            Ok(())
        } else {
            Err(Error::new(ErrorKind::InvalidData, "unexpected response"))
        }
    }

    async fn write_multiple_coils(
        &mut self,
        slave_id: u8,
        address: u16,
        data: &[bool],
    ) -> Result<(), Error> {
        let rsp = Self::call(self, slave_id, Request::WriteMultipleCoils(address, data)).await?;

        if let Response::WriteMultipleCoils(rsp_addr, rsp_cnt) = rsp {
            if rsp_addr != address || data.len() != rsp_cnt as usize {
                return Err(Error::new(ErrorKind::InvalidData, "invalid response"));
            }
            Ok(())
        } else {
            Err(Error::new(ErrorKind::InvalidData, "unexpected response"))
        }
    }

    async fn write_multiple_registers(
        &mut self,
        slave_id: u8,
        address: u16,
        data: &[u16],
    ) -> Result<(), Error> {
        let rsp = Self::call(
            self,
            slave_id,
            Request::WriteMultipleRegisters(address, data),
        )
        .await?;

        if let Response::WriteMultipleRegisters(rsp_addr, rsp_cnt) = rsp {
            if rsp_addr != address || data.len() != rsp_cnt as usize {
                return Err(Error::new(ErrorKind::InvalidData, "invalid response"));
            }
            Ok(())
        } else {
            Err(Error::new(ErrorKind::InvalidData, "unexpected response"))
        }
    }
}
