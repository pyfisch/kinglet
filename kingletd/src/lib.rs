extern crate mio;
extern crate rotor;
extern crate httparse;
extern crate kingletp;

pub mod http1;

use rotor::transports::{accept, stream};
pub use mio::tcp::{TcpListener, TcpStream};
pub use mio::EventLoop;

pub use rotor::Handler as EventHandler;
pub use http1::Handler;
pub use kingletp::{Method, HttpVersion, StatusCode, Url, Headers, Message, Request, Response};

pub type HttpServer<C, R> = accept::Serve<C,
                        TcpListener,
                        stream::Stream<C, TcpStream, http1::Client<C, R>>>;
