//! A Hello World example application for working with Gotham.

extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;

use hyper::{Response, StatusCode, Get, Head};

use gotham::http::response::create_response;
use gotham::state::State;

use gotham::router::Router;
use gotham::router::builder::*;


fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(say_hello);
    })
}

pub fn say_hello(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from(r#"{
            "some_string": 69
        }"#).into_bytes(), mime::APPLICATION_JSON)),
    );

    (state, res)
}

pub fn main() {
    let addr = "127.0.0.1:8080";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}
