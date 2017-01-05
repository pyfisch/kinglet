use std::io;

use futures::{Poll, StartSend};
use futures::stream::Stream;
use futures::sink::Sink;
use tokio_proto::streaming::pipeline::{Frame, Transport};

use {Request, Response, Chunk};

// PhantomData is a mere placeholder until I know what to do with T
pub struct Http1Transport<T>(::std::marker::PhantomData<T>);

impl<T> Stream for Http1Transport<T> {
    type Item = Frame<Request, Chunk, io::Error>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        unimplemented!();
    }
}

impl<T> Sink for Http1Transport<T> {
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

impl<T: 'static> Transport for Http1Transport<T> {}
