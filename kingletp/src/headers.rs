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
