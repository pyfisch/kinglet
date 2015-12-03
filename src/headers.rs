use std::borrow::Cow;
use std::mem;
use std::ops::Deref;
use std::fmt::Debug;

use httparse::Header;
use multimap::MultiMap;
use unicase::UniCase;

#[derive(Debug, PartialEq)]
pub struct Headers {
    inner: MultiMap<UniCase<Cow<'static, str>>, Vec<u8>>,
}

impl Headers {
    pub fn new() -> Self {
        Headers { inner: MultiMap::new() }
    }

    pub fn from_http1(raw: &[Header]) -> Self {
        let mut headers = Headers::new();
        for header in raw {
            headers.insert_http1_header(header)
        }
        headers
    }

    pub fn insert_http1_header(&mut self, header: &Header) {
        self.insert(header.name.to_owned(), header.value.to_vec());
    }

    pub fn insert<K: Into<Cow<'static, str>> + Debug>(&mut self, name: K, value: Vec<u8>) {
        self.inner.insert(UniCase(name.into()), value)
    }

    pub fn get_vec(&self, name: &str) -> Option<&Vec<Vec<u8>>> {
        self.inner.get_vec(&UniCase(Cow::Borrowed(unsafe { mem::transmute::<&str, &str>(name) })))
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.inner
            .contains_key(&UniCase(Cow::Borrowed(unsafe { mem::transmute::<&str, &str>(name) })))
    }
}

impl Deref for Headers {
    type Target = MultiMap<UniCase<Cow<'static, str>>, Vec<u8>>;

    fn deref<'a>(&'a self) -> &'a MultiMap<UniCase<Cow<'static, str>>, Vec<u8>> {
        &self.inner
    }
}

pub struct IterListHeader<'a> {
    values: &'a Vec<Vec<u8>>,
    line: usize,
    column: usize,
}

impl <'a>IterListHeader<'a> {
    pub fn new(values: &Vec<Vec<u8>>) -> IterListHeader {
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
        for line in self.line..self.values.len() {
            let value = &self.values[line];
            let mut maybe_start_column = None;
            let mut end_column = 0;
            for column in self.column..value.len() {
                let byte = value[column];
                if byte != b' ' && byte != b'\t' && byte != b',' {
                    end_column = column + 1;
                    if maybe_start_column.is_none() {
                        maybe_start_column = Some(column)
                    }
                } else if byte == b',' {
                    if let Some(start_column) = maybe_start_column {
                        self.column = column + 1;
                        return Some(&value[start_column..end_column]);
                    }
                    maybe_start_column = None;
                }
            }
            self.line = line + 1;
            self.column = 0;
            if let Some(start_column) = maybe_start_column {
                return Some(&value[start_column..end_column]);
            }
        }
        None
    }
}
