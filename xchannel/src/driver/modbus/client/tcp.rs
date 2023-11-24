use std::{
    io::{Error, ErrorKind},
    sync::atomic::{AtomicU16, Ordering},
};

use async_trait::async_trait;
use futures_util::{sink::SinkExt as _, stream::StreamExt as _};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::Framed;

use super::super::protocol::{
    Request, RequestPdu, Response, ResponsePdu,
    tcp::{Header, ClientCodec, RequestAdu},
};

use super::Client;

struct AsyncTcpClient<T> {
    framed: Framed<T, ClientCodec>,
    transaction_id: AtomicU16,
    //TODO req list
}

impl<T> AsyncTcpClient<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(transport: T) -> Self {
        AsyncTcpClient {
            framed: Framed::new(transport, ClientCodec::default()),
            transaction_id: AtomicU16::new(0),
        }
    }

    pub fn next_transaction_id(&self) -> u16 {
        let transaction_id = self.transaction_id.load(Ordering::Relaxed);
        self.transaction_id
            .store(transaction_id.wrapping_add(1), Ordering::Relaxed);

        transaction_id
    }

    pub fn next_request_adu<'a, R>(&self, slave_id: u8, req: R) -> RequestAdu<'a>
    where
        R: Into<RequestPdu<'a>>,
    {
        let header = Header {
            transaction_id: self.next_transaction_id(),
            unit_id: slave_id,
        };
        RequestAdu {
            header,
            pdu: req.into(),
        }
    }
}

#[async_trait]
impl<T> Client for AsyncTcpClient<T>
where
    T: Send + AsyncRead + AsyncWrite + Unpin,
{
    async fn call(&mut self, slave_id: u8, request: Request<'_>) -> Result<Response, Error> {
        let req_adu = self.next_request_adu(slave_id, request);
        let _req_hdr = req_adu.header;

        self.framed.read_buffer_mut().clear();

        self.framed.send(req_adu).await?;
        let res_adu = self
            .framed
            .next()
            .await
            .ok_or_else(|| Error::last_os_error())??;

        match res_adu.pdu {
            ResponsePdu(Ok(_res)) => Err(Error::new(ErrorKind::InvalidData, "unexpected response")),
            ResponsePdu(Err(err)) => Err(Error::new(ErrorKind::Other, err)),
        }
    }
}
