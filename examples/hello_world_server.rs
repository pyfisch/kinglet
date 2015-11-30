extern crate kingletd;

use kingletd::{Handler, HttpServer, Request, Response, Url, Message, TcpListener, EventLoop, EventHandler};

struct Context {
    counter: usize,
}

trait Counter {
    fn increment(&mut self);
    fn get(&self) -> usize;
}

impl Counter for Context {
    fn increment(&mut self) { self.counter += 1; }
    fn get(&self) -> usize { self.counter }
}

struct HelloWorld;

impl<C:Counter> Handler<C> for HelloWorld {
    fn request(req: Request, ctx: &mut C) -> Response {
        let mut res = Response::new(req.version);
        ctx.increment();
        let request_path: String = req.request_url().ok().as_ref().and_then(Url::serialize_path).unwrap();
        match &request_path[..] {
            "/" => {
                res.put_body("Hello World!");
            }
            "/num" => {
                res.put_body(format!("This host visited {} times",
                                     ctx.get()));
            }
            "/body" => {
                if req.is_chunked() {
                    res.put_body(format!("You sent a message with this chunked body:\n{}\n", String::from_utf8(req.body).unwrap()));
                } else {
                    res.put_body(format!("You sent a message with this body:\n{}\n", String::from_utf8(req.body).unwrap()));
                }
            }
            p => {
                res.put_body(format!("Hello {}!", &p[1..]));
            }
        }
        res
    }
}

fn main() {
    let tcp_listener = TcpListener::bind(&"127.0.0.1:8888".parse().unwrap()).unwrap();
    let mut event_loop = EventLoop::new().unwrap();
    let mut handler = EventHandler::new(Context { counter: 0 }, &mut event_loop);
    handler.add_root(&mut event_loop, HttpServer::<_, HelloWorld>::new(tcp_listener));
    event_loop.run(&mut handler).unwrap();
}
