use std::str;
use std::str::FromStr;
use std::ascii::AsciiExt;

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

pub struct IterListHeader<'a> {
    values: &'a Vec<Vec<u8>>,
    line: usize,
    column: usize,
}

impl <'a>IterListHeader<'a> {
    fn new(values: &Vec<Vec<u8>>) -> IterListHeader {
        IterListHeader {
            values: values,
            line: 0,
            column: 0,
        }
    }
}

impl <'a>Iterator for IterListHeader<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<&'a [u8]> {
        for i in self.line..self.values.len() {
            let mut start = true;
            let value = &self.values[i];
            for j in self.column..value.len() {
                if start && (value[j] == b' ' || value[j] == b'\t') {
                    self.column = j + 1;
                } else if start {
                    start = false;
                }
                if value[j] == b',' {
                    let val = &value[self.column..j];
                    self.column = j + 1;
                    return Some(val);
                }
            }
            if self.column < value.len() {
                let val = &value[self.column..];
                self.column = value.len();
                return Some(val);
            }
            self.line += 1;
            self.column = 0;
        }
        None
    }
}
