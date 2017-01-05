use std::io;

use tokio_core::net::TcpStream;
use tokio_proto::streaming::pipeline::ServerProto;

use {Request, Response, Chunk, Http1Transport};

/// The HTTP/1.x protocol.
// opaque for future extension
pub struct Http1(());

impl ServerProto<TcpStream> for Http1 {
    type Request = Request;
    type RequestBody = Chunk;
    type Response = Response;
    type ResponseBody = Chunk;
    type Error = io::Error;
    type Transport = Http1Transport;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: TcpStream) -> Self::BindTransport {
        Ok(Http1Transport::new(io))
    }
}
