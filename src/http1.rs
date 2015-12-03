use std::cmp;
use std::error::Error;
use std::marker::PhantomData;
use std::str;

use rotor::transports::stream::{Transport, Protocol};
use rotor::transports::StreamSocket;
use rotor::buffer_util::find_substr;
use rotor::async::Async;
use httparse;
use Message;
use Request;
use Response;


/// Note httparse requires we preallocate array of this size so be wise
pub const MAX_HEADERS_NUM: usize = 256;
/// This one is not preallocated, but too large buffer is of limited use
/// because of previous parameter.
pub const MAX_HEADERS_SIZE: usize = 16384;
/// This is not "enough for everyone" but we probably need some limit anyway.
/// Note that underlying `netbuf` impl limited to a little less than 4GiB
pub const MAX_BODY_SIZE: usize = 104_856_700;


pub trait Handler<C> {
    /// Dispatched when request arrives.
    ///
    /// We don't support POST body yet, so this is only one callback, but will
    /// probably be split into many in future
    fn request(_request: Request, _ctx: &mut C) -> Response;
}

/// A connection with a client.
///
/// The `Initial`, `KeepAlive` and `ReadHeaders` states are kept separate for
/// debugging and different timeouts in future eventuallly.
#[derive(Debug, PartialEq)]
pub enum Client<C, H: Handler<C>> {
    /// The initial state of a connection.
    Initial,
    /// The state after some headers have been read.
    ReadHeaders, // TODO(tailhook) 100 Expect?
    /// Reading a request body with a fixed size.
    ///
    /// The `usize` gives the number of remaining bytes.
    ReadFixedSize(Request, usize),
    /// Reading a request body in chunked encoding.
    ///
    /// The value describes the remaining size of the chunk. No value means there is no chunk
    /// parsed currently. Zero means the chunk was completed but the terminator of the chunk
    /// was not yet parsed.
    ReadChunked(Request, Option<usize>),
    /// Read the trailing header fields after a chunked encoding.
    ReadTrailers(Request),
    /// A complete request.
    Parsed(Request),
    /// A connection in idle state.
    KeepAlive,

    #[doc(hidden)]
    __Handler(PhantomData<(C, H)>),
}

fn parse_headers(transport: &mut Transport) -> Result<Option<Request>, Box<Error + Send + Sync>> {
    let mut buf = transport.input();
    let headers_end = match find_substr(&buf[..], b"\r\n\r\n") {
        Some(x) => x,
        None => {
            return Ok(None);
        }
    };
    let mut headers = [httparse::EMPTY_HEADER; MAX_HEADERS_NUM];
    let req = {
        let mut raw = httparse::Request::new(&mut headers);
        match raw.parse(&buf[..]) {
            Ok(httparse::Status::Complete(x)) => {
                assert!(x == headers_end + 4);
            }
            Ok(_) => unreachable!(),
            Err(_) => {
                return Err(From::from("Header syntax mismatch"));
            }
        }
        Request::from_http1(raw, false).expect("a valid request")
    };
    buf.consume(headers_end + 4);
    Ok(Some(req))
}

fn parse_chunk_size(transport: &mut Transport) -> Result<Option<usize>, ()> {
    fn is_lower_hexdigit(chr: u8) -> bool {
        (chr >= 0x30 && chr <= 0x39) || (chr >= 0x61 && chr <= 0x66)
    }
    let mut buf = transport.input();
    let mut hexdigit_end = 0;
    // let mut has_ext = false;
    for (i, byte) in buf[..].iter().enumerate() {
        if byte == &b'\r' {
            hexdigit_end = i;
            break;
        } else if byte == &b';' {
            hexdigit_end = i;
            // has_ext = true;
            break;
        } else if !is_lower_hexdigit(*byte) {
            return Err(());
        }
    }
    if buf[..][hexdigit_end + 1] != b'\n' {
        return Err(());
    }
    let chunk_size = usize::from_str_radix(str::from_utf8(&buf[..hexdigit_end]).unwrap(), 16)
                         .unwrap();
    buf.consume(hexdigit_end + 2);
    Ok(Some(chunk_size))
}

fn parse_fixed_size(transport: &mut Transport, store: &mut Vec<u8>, mut size: usize) -> usize {
    let mut buf = transport.input();
    let size_read = cmp::min(size, buf.len());
    size -= size_read;
    store.extend(&buf[..size_read]);
    buf.consume(size_read);
    size
}

impl<C, H: Handler<C>> Protocol<C> for Client<C, H> {
    fn accepted<S: StreamSocket>(_conn: &mut S, _context: &mut C) -> Option<Self> {
        Some(Client::Initial)
    }
    fn data_received(mut self, transport: &mut Transport, ctx: &mut C) -> Async<Self, ()> {
        use self::Client::*;
        loop {
            self = match self {
                Initial | ReadHeaders | KeepAlive => {
                    match parse_headers(transport) {
                        Err(_) => return Async::Stop,
                        Ok(None) => return Async::Continue(ReadHeaders, ()),
                        Ok(Some(req)) => {
                            if let Ok(length) = req.content_length() {
                                ReadFixedSize(req, length)
                            } else if req.is_chunked() {
                                ReadChunked(req, None)
                            } else {
                                Parsed(req)
                            }
                        }
                    }
                }
                ReadFixedSize(mut req, size) => {
                    match parse_fixed_size(transport, &mut req.body, size) {
                        0 => Parsed(req),
                        x => return return Async::Continue(ReadFixedSize(req, x), ()),
                    }
                }
                ReadChunked(req, None) => {
                    match parse_chunk_size(transport) {
                        Err(_) => return Async::Stop,
                        Ok(None) => return Async::Continue(ReadChunked(req, None), ()),
                        Ok(Some(0)) => {
                            let mut buf = transport.input();
                            if buf.len() >= 2 && &buf[..2] == b"\r\n" {
                                // no trailers found
                                buf.consume(2);
                                Parsed(req)
                            } else {
                                ReadTrailers(req)
                            }
                        }
                        Ok(Some(chunk_size)) => ReadChunked(req, Some(chunk_size)),
                    }
                }
                ReadChunked(req, Some(0)) => {
                    let mut buf = transport.input();
                    if buf.len() < 2 {
                        return Async::Continue(ReadChunked(req, Some(0)), ());
                    }
                    if &buf[..2] != b"\r\n" {
                        return Async::Stop;
                    }
                    buf.consume(2);
                    ReadChunked(req, None)
                }
                ReadChunked(mut req, Some(mut size)) => {
                    size = parse_fixed_size(transport, &mut req.body, size);
                    ReadChunked(req, Some(size))
                }
                ReadTrailers(mut req) => {
                    use httparse::Status::*;
                    let mut buf = transport.input();
                    let mut headers = [httparse::EMPTY_HEADER; MAX_HEADERS_NUM];
                    let consumed_len = match httparse::parse_headers(&buf[..], &mut headers) {
                        Ok(Complete((consumed_len, headers))) => {
                            req.add_http1_headers(headers);
                            consumed_len
                        }
                        Ok(Partial) => return Async::Continue(ReadTrailers(req), ()),
                        Err(_) => return Async::Stop,
                    };
                    buf.consume(consumed_len);
                    Parsed(req)
                }
                Parsed(req) => {
                    let res = <H as Handler<C>>::request(req, ctx);
                    let buf = transport.output();
                    if let Err(_) = res.serialize(buf) {
                        return Async::Stop;
                    }
                    KeepAlive
                }
                _ => unimplemented!(),
            };
            if let Parsed(_) = self {
                continue;
            }
            if transport.input().empty() {
                return Async::Continue(self, ());
            }
        }
    }
}
