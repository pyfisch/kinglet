use std::io;

use tokio_core::io::Io;
use tokio_proto::streaming::pipeline::ServerProto;

use {Request, Response, Chunk, Http1Transport};

/// The HTTP/1.x protocol.
// opaque for future extension
pub struct Http1(());

impl<T: Io + 'static> ServerProto<T> for Http1 {
    type Request = Request;
    type RequestBody = Chunk;
    type Response = Response;
    type ResponseBody = Chunk;
    type Error = io::Error;
    type Transport = Http1Transport<T>;
    type BindTransport = Result<Self::Transport, io::Error>;
    
    fn bind_transport(&self, _io: T) -> Self::BindTransport {
        unimplemented!();
    }
}
