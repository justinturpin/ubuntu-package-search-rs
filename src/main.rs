//! A Hello World example application for working with Gotham.

extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate futures;

use hyper::{Response, StatusCode};

use gotham::http::response::create_response;
use gotham::state::State;
use gotham::handler::HandlerFuture;

use gotham::router::Router;
use gotham::router::builder::*;

use futures::{Future, Stream};
use reqwest::async::{Client, Decoder};

use std::mem;
use std::io;


fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(say_hello);
    })
}

pub fn request_async(url: String) -> Box<Future<Item=Vec<u8>, Error=()>> {
    let request = reqwest::async::Client::new().get(
        "http://httpbin.org"
    )
    .send()
    .and_then(|mut res| {
        println!("{}", res.status());

        let body = mem::replace(res.body_mut(), Decoder::empty());
        body.concat2()
    })
    .map_err(|err| println!("request error: {}", err))
    .map(|body| {
        let mut cursor = std::io::Cursor::new(body);
        let mut buffer: Vec<u8> = vec![];
        io::copy(&mut cursor, &mut buffer);

        buffer
    });

    Box::new(request)
}

pub fn say_hello(state: State) -> Box<HandlerFuture> {
    let request = request_async(String::from("http://httpbin.org"));

    Box::new(request.then(|result| {
        let res = create_response(
            &state, StatusCode::Ok, Some((result.unwrap(), mime::TEXT_PLAIN))
        );

        Ok((state, res))
    }))
}

pub fn main() {
    let addr = "127.0.0.1:8080";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(router()))
}
