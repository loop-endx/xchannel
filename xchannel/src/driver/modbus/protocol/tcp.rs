use std::io::{Error, ErrorKind};

use byteorder::{BigEndian, ByteOrder};
use bytes::{BufMut, Bytes};
use tokio_util::codec::{Decoder, Encoder};

use super::{Request, Response};

const HEADER_LEN: usize = 7;
const PROTOCOL_ID: u16 = 0x0000;

#[derive(Clone, Copy)]
pub struct Header {
    pub transaction_id: u16,
    pub unit_id: u8,
}

#[derive(Clone)]
pub struct RequestAdu<'a> {
    pub header: Header,
    pub request: Request<'a>,
}

pub struct ResponseAdu {
    pub header: Header,
    pub response: Response,
}
pub struct AduDecoder;
pub struct ClientCodec {
    pub decoder: AduDecoder,
}

impl Default for ClientCodec {
    fn default() -> Self {
        ClientCodec {
            decoder: AduDecoder,
        }
    }
}

impl Decoder for AduDecoder {
    type Item = (Header, Bytes);
    type Error = Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < HEADER_LEN {
            return Ok(None);
        }

        let len = BigEndian::read_u16(&src[4..6]);
        if src.len() < HEADER_LEN + len as usize - 1 {
            return Ok(None);
        }

        let protocol_id = BigEndian::read_u16(&src[2..4]);
        if protocol_id != PROTOCOL_ID {
            return Err(Error::new(ErrorKind::InvalidData, "invalid protocol id"));
        }

        let transaction_id = BigEndian::read_u16(&src[0..2]);
        let unit_id = src[6];
        let _header = src.split_to(HEADER_LEN);
        let pdu_data = src.split_to(len as usize - 1).freeze();

        let header = Header {
            transaction_id,
            unit_id,
        };

        Ok(Some((header, pdu_data)))
    }
}

impl Decoder for ClientCodec {
    type Item = ResponseAdu;
    type Error = Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some((header, pdu_data)) = self.decoder.decode(src)? {
            let response = Response::try_from(pdu_data)?;
            Ok(Some(ResponseAdu { header, response }))
        } else {
            Ok(None)
        }
    }
}

impl<'a> Encoder<RequestAdu<'a>> for ClientCodec {
    type Error = Error;

    fn encode(
        &mut self,
        item: RequestAdu<'a>,
        dst: &mut bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        let RequestAdu { header, request } = item;
        let pdu_data: Bytes = request.into();
        dst.reserve(pdu_data.len() + 7);
        dst.put_u16(header.transaction_id);
        dst.put_u16(PROTOCOL_ID);
        dst.put_u16(pdu_data.len() as u16 + 1);
        dst.put_u8(header.unit_id);
        dst.put_slice(&pdu_data);
        Ok(())
    }
}
