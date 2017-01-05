extern crate futures;
extern crate httparse;
extern crate httpdate;
extern crate httptypes;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

pub use proto::Http1;
pub use transport::Http1Transport;

mod proto;
mod transport;

pub struct Request(());
pub struct Response(());
pub struct Chunk(());

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
