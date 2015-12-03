extern crate netbuf;
extern crate rotor;
extern crate kinglet;

use netbuf::Buf;
use rotor::async::Async;
use rotor::transports::stream::{Transport, Protocol};
use kinglet::{Request, Response, Url, HttpVersion, Method, Message};
use kinglet::http1::{Client, Handler};

#[test]
fn parse_get_request() {
    #[derive(Debug, Eq, PartialEq)]
    struct DummyHandler;
    impl Handler<()> for DummyHandler {
        fn request(req: Request, _: &mut ()) -> Response {
            assert_eq!(req.request_url().unwrap(), Url::parse("http://example.org/").unwrap());
            assert_eq!(req.method, Method::Get);
            Response::new(HttpVersion::Http11)
        }
    }
    let mut inbuf = Buf::new();
    inbuf.extend(b"GET / HTTP/1.1\r\nHost: example.org\r\n\r\n");
    let mut outbuf = Buf::new();
    {
        let mut transport = Transport::new(&mut inbuf, &mut outbuf);
        let client = Client::Initial::<(), DummyHandler>;
        assert_eq!(client.data_received(&mut transport, &mut ()), Async::Continue(Client::KeepAlive, ()));
    }
    assert!(inbuf.empty());
}

#[test]
fn parse_post_request() {
    #[derive(Debug, Eq, PartialEq)]
    struct DummyHandler;
    impl Handler<()> for DummyHandler {
        fn request(req: Request, _: &mut ()) -> Response {
            assert_eq!(req.request_url().unwrap(), Url::parse("http://example.org/create").unwrap());
            assert_eq!(req.method, Method::Post);
            assert_eq!(req.body, b"foobar");
            Response::new(HttpVersion::Http11)
        }
    }
    let mut inbuf = Buf::new();
    inbuf.extend(b"POST /create HTTP/1.1\r\nHost: example.org\r\nContent-Length: 6\r\n\r\nfoobar");
    let mut outbuf = Buf::new();
    {
        let mut transport = Transport::new(&mut inbuf, &mut outbuf);
        let client = Client::Initial::<(), DummyHandler>;
        assert_eq!(client.data_received(&mut transport, &mut ()), Async::Continue(Client::KeepAlive, ()));
    }
    assert!(inbuf.empty());
}

#[test]
fn parse_chunked_request() {
    #[derive(Debug, Eq, PartialEq)]
    struct DummyHandler;
    impl Handler<()> for DummyHandler {
        fn request(req: Request, _: &mut ()) -> Response {
            assert_eq!(req.request_url().unwrap(), Url::parse("http://example.org/create").unwrap());
            assert_eq!(req.method, Method::Post);
            assert_eq!(req.body, b"foobar");
            Response::new(HttpVersion::Http11)
        }
    }
    let mut inbuf = Buf::new();
    inbuf.extend(b"POST /create HTTP/1.1\r\nHost: example.org\r\nTransfer-Encoding: chunked\r\n\r\n2\r\nfo\r\n4\r\nobar\r\n0\r\n\r\n");
    let mut outbuf = Buf::new();
    let client = Client::Initial::<(), DummyHandler>;
    {
        let mut transport = Transport::new(&mut inbuf, &mut outbuf);
        assert_eq!(client.data_received(&mut transport, &mut ()), Async::Continue(Client::KeepAlive, ()));
    }
    assert!(inbuf.empty());
}

#[test]
fn parse_chunked_wikipedia() {
    #[derive(Debug, Eq, PartialEq)]
    struct DummyHandler;
    impl Handler<()> for DummyHandler {
        fn request(req: Request, _: &mut ()) -> Response {
            assert_eq!(req.request_url().unwrap(), Url::parse("http://wikipedia.org/wiki").unwrap());
            assert_eq!(req.method, Method::Post);
            assert_eq!(req.body, b"Wikipedia in\r\n\r\nchunks.");
            Response::new(HttpVersion::Http11)
        }
    }
    let mut inbuf = Buf::new();
    inbuf.extend(b"POST /wiki HTTP/1.1\r\nHost: wikipedia.org\r\nTransfer-Encoding: chunked\r\n\r\n4\r\nWiki\r\n5\r\npedia\r\ne\r\n in\r\n\r\nchunks.\r\n0\r\n\r\n");
    let mut outbuf = Buf::new();
    let client = Client::Initial::<(), DummyHandler>;
    {
        let mut transport = Transport::new(&mut inbuf, &mut outbuf);
        assert_eq!(client.data_received(&mut transport, &mut ()), Async::Continue(Client::KeepAlive, ()));
    }
    assert!(inbuf.empty());
}

#[test]
fn parse_chunked_wikipedia_many_packets() {
    #[derive(Debug, Eq, PartialEq)]
    struct DummyHandler;
    impl Handler<()> for DummyHandler {
        fn request(req: Request, _: &mut ()) -> Response {
            assert_eq!(req.request_url().unwrap(), Url::parse("http://wikipedia.org/wiki").unwrap());
            assert_eq!(req.method, Method::Post);
            assert_eq!(req.body, b"Wikipedia in\r\n\r\nchunks.");
            Response::new(HttpVersion::Http11)
        }
    }
    let mut inbuf = Buf::new();
    let mut outbuf = Buf::new();
    let client = Client::Initial::<(), DummyHandler>;
    inbuf.extend(b"POST /wiki HTTP/1.1\r\nHo");
    assert_eq!({
        let mut transport = Transport::new(&mut inbuf, &mut outbuf);
        client.data_received(&mut transport, &mut ())
    }.and_then(|client| {
        inbuf.extend(b"st: wikipedia.org\r\n");
        let mut transport = Transport::new(&mut inbuf, &mut outbuf);
        client.data_received(&mut transport, &mut ())
    }).and_then(|client| {
        inbuf.extend(b"Transfer-Encoding: chunked\r\n");
        let mut transport = Transport::new(&mut inbuf, &mut outbuf);
        client.data_received(&mut transport, &mut ())
    }).and_then(|client| {
        inbuf.extend(b"\r\n4\r\nWi");
        let mut transport = Transport::new(&mut inbuf, &mut outbuf);
        client.data_received(&mut transport, &mut ())
    }).and_then(|client| {
        inbuf.extend(b"ki\r\n5\r\npedia\r");
        let mut transport = Transport::new(&mut inbuf, &mut outbuf);
        client.data_received(&mut transport, &mut ())
    }).and_then(|client| {
        inbuf.extend(b"\ne\r\n in\r\n\r\nchunks.\r\n0\r\n\r\n");
        let mut transport = Transport::new(&mut inbuf, &mut outbuf);
        client.data_received(&mut transport, &mut ())
    }), Async::Continue(Client::KeepAlive, ()));
    assert!(inbuf.empty());
}

#[test]
fn parse_pipelineing() {
    #[derive(Debug, Eq, PartialEq)]
    struct DummyHandler;
    impl Handler<()> for DummyHandler {
        fn request(_: Request, _: &mut ()) -> Response {
            Response::new(HttpVersion::Http11)
        }
    }
    let mut inbuf = Buf::new();
    let mut outbuf = Buf::new();
    let client = Client::Initial::<(), DummyHandler>;
    inbuf.extend(b"GET /foo HTTP/1.1\r\nHost: example.com\r\n\r\nGET /bar HTTP/1.1\r\nHost: example.com\r\n\r\n");
    {
        let mut transport = Transport::new(&mut inbuf, &mut outbuf);
        assert_eq!(client.data_received(&mut transport, &mut ()), Async::Continue(Client::KeepAlive, ()));
    }
    assert!(inbuf.empty());
}

#[test]
fn trailing_header_fields() {
    #[derive(Debug, Eq, PartialEq)]
    struct DummyHandler;
    impl Handler<()> for DummyHandler {
        fn request(req: Request, _: &mut ()) -> Response {
            assert!(req.contains_header("X-Header"));
            Response::new(HttpVersion::Http11)
        }
    }
    let mut inbuf = Buf::new();
    let mut outbuf = Buf::new();
    let client = Client::Initial::<(), DummyHandler>;
    inbuf.extend(b"POST /x HTTP/1.1\r\nHost: foo.bar\r\nTransfer-Encoding: cHUnked\r\n\r\n2\r\nxy\r\n0\r\nX-Header: true\r\n\r\n");
    {
        let mut transport = Transport::new(&mut inbuf, &mut outbuf);
        assert_eq!(client.data_received(&mut transport, &mut ()), Async::Continue(Client::KeepAlive, ()));
    }
    assert!(inbuf.empty());
}
