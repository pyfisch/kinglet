extern crate httparse;
extern crate hyper;
extern crate mio;
extern crate netbuf;
extern crate rotor;
extern crate unicase;
extern crate url;
extern crate time;
extern crate multimap;

use rotor::transports::{accept, stream};

pub use hyper::method::Method;
pub use hyper::status::StatusCode;
pub use hyper::version::HttpVersion;
pub use mio::{EventLoop};
pub use mio::tcp::{TcpListener, TcpStream};
pub use rotor::Handler as EventHandler;
pub use url::Url;

pub use error::{Error, Result};
pub use headers::{IterListHeader, Headers};
pub use http1::Handler;
pub use message::Message;
pub use request::Request;
pub use response::Response;

mod error;
mod headers;
pub mod http1;
mod message;
mod request;
mod response;

pub type HttpServer<C, R> = accept::Serve<C,
                        TcpListener,
                        stream::Stream<C, TcpStream, http1::Client<C, R>>>;
