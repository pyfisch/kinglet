use std::str;

use url::{ParseResult, Url};

use Error::{InvalidVersion, InvalidMethod, InvalidMessage};
use Headers;
use HttpVersion::{self, Http09, Http10, Http11, Http20};
use Method;
use Message;
use httparse::{self, Header};

#[derive(Debug, PartialEq)]
pub struct Request {
    /// HTTP version used in the request.
    pub version: HttpVersion,
    /// HTTP verb used in the request.
    pub method: Method,
    scheme: String,
    authority: Option<String>,
    path: String,
    headers: Headers,
    pub body: Vec<u8>,
}

impl Request {
    pub fn from_http1(raw: httparse::Request, secure: bool) -> ::Result<Self> {
        Ok(Request {
            version: if try!(raw.version.ok_or(InvalidVersion)) == 1 {
                Http11
            } else {
                Http10
            },
            method: try!(try!(raw.method.ok_or(InvalidMethod)).parse().or(Err(InvalidMessage))),
            scheme: if secure {
                "https".to_owned()
            } else {
                "http".to_owned()
            },
            authority: None,
            path: try!(raw.path.ok_or(InvalidMessage)).to_owned(),
            headers: Headers::from_http1(raw.headers),
            body: Vec::new(),
        })
    }

    pub fn add_http1_headers(&mut self, raw: &[Header]) {
        for header in raw {
            self.headers.insert_http1_header(header);
        }
    }

    pub fn request_url(&self) -> ::Result<Url> {
        // TODO: Support `*` paths
        fn url_from_http(scheme: &str, authority: &str, path: &str) -> ParseResult<Url> {
            Url::parse(&format!("{}://{}{}", scheme, authority, path)[..])
        }
        match self.version {
            Http09 | Http10 => url_from_http(&self.scheme[..], "0.0.0.0", &self.path[..]),
            Http11 => {
                let host = try!(str::from_utf8(try!(self.get_value_header("Host")
                                                        .ok_or(InvalidMessage))));
                Url::parse(&self.path[..]).or(url_from_http(&self.scheme[..], host, &self.path[..]))
            }
            Http20 => {
                url_from_http(&self.scheme[..],
                              &try!(self.authority.as_ref().ok_or(InvalidMessage))[..],
                              &self.path[..])
            }
        }
        .map_err(From::from)
    }
}

impl Message for Request {
    fn get_header(&self, name: &str) -> Option<&Vec<Vec<u8>>> {
        self.headers.get_vec(name)
    }

    fn contains_header(&self, name: &str) -> bool {
        self.headers.contains_key(name)
    }
}
