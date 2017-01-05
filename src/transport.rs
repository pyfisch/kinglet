use std::io;

use futures::{Poll, StartSend};
use futures::{Stream, Sink};
use httparse;
use tokio_core::net::TcpStream;
use tokio_proto::streaming::pipeline::{Frame, Transport};

use {Request, Response, Chunk};

pub struct Http1Transport {
    io: TcpStream,
}

impl Http1Transport {
    pub fn new(io: TcpStream) -> Http1Transport {
        Http1Transport {
            io: io,
        }
    }
}

impl Stream for Http1Transport {
    type Item = Frame<Request, Chunk, io::Error>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut raw_req = httparse::Request::new(&mut headers);
        // Read the request header from the stream.
        // Transform the Request
        // Return an Item.
        unimplemented!();
    }
}

impl Sink for Http1Transport {
    type SinkItem = Frame<Response, Chunk, io::Error>;
    type SinkError = io::Error;
    fn start_send(&mut self,
                  _item: Self::SinkItem)
                  -> StartSend<Self::SinkItem, Self::SinkError> {
        unimplemented!();
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        unimplemented!();
    }
}

impl Transport for Http1Transport {}
