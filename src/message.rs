use std::str;
use std::str::FromStr;
use std::ascii::AsciiExt;

use IterListHeader;
use Error::{ForbiddenHeader, MissingHeader};

pub trait Message {
    fn get_header(&self, &str) -> Option<&Vec<Vec<u8>>>;
    fn contains_header(&self, &str) -> bool;

    fn get_value_header(&self, name: &str) -> Option<&[u8]> {
        let value_header = self.get_header(name);
        if value_header.is_none() || value_header.unwrap().len() != 1 {
            return None;
        }
        Some(&value_header.unwrap()[0])
    }

    fn get_list_header(&self, name: &str) -> Option<IterListHeader> {
        if let Some(values) = self.get_header(name) {
            Some(IterListHeader::new(values))
        } else {
            None
        }
    }

    fn content_length(&self) -> ::Result<usize> {
        if self.contains_header("Transfer-Encoding") {
            return Err(ForbiddenHeader);
        }
        let value = try!(self.get_value_header("Content-Length").ok_or(MissingHeader));
        FromStr::from_str(try!(str::from_utf8(value))).map_err(From::from)
    }

    fn is_chunked(&self) -> bool {
        if let Some(values) = self.get_list_header("Transfer-Encoding") {
            for value in values {
                if value.eq_ignore_ascii_case(b"chunked") {
                    return true
                }
            }
        }
        false
    }
}
