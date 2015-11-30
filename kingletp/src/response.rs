use std::io::{self, Write};

use Headers;
use HttpVersion;
use Message;
use StatusCode;
use time;

#[derive(Debug)]
pub struct Response {
    pub version: HttpVersion,
    pub status: StatusCode,
    reason: Option<String>,
    headers: Headers,
    body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(version: HttpVersion) -> Self {
        Response {
            version: version,
            status: StatusCode::Ok,
            reason: None,
            headers: Headers::new(),
            body: None
        }
    }

    pub fn put_body<B:AsRef<[u8]>>(&mut self, body: B) {
        self.body = Some(body.as_ref().to_owned());
    }

    pub fn serialize<W: Write>(&self, mut w: &mut W) -> io::Result<()> {
        // TODO: custom status.
        try!(write!(&mut w, "{} {}\r\n", self.version, self.status));
        if !self.contains_header("Date") {
            try!(write!(&mut w, "Date: {}\r\n", time::now().rfc822()));
        }
        for (name, value) in self.headers.iter_all() {
            for v in value {
                try!(write!(&mut w, "{}: ", name));
                try!(w.write_all(&v[..]));
                try!(w.write_all(b"\r\n"));
            }
        }
        if let Some(ref body) = self.body {
            try!(write!(w, "Content-Length: {}\r\n", body.len()));
            try!(w.write_all(b"\r\n"));
            try!(w.write_all(&body[..]));
        } else {
            try!(w.write_all(b"\r\n"));
        }
        Ok(())
    }
}

impl Message for Response {
    fn get_header(&self, name: &str) -> Option<&Vec<Vec<u8>>> {
        self.headers.get_vec(name)
    }

    fn contains_header(&self, name: &str) -> bool {
        self.headers.contains_key(name)
    }
}
