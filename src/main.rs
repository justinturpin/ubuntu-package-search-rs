//! A Hello World example application for working with Gotham.

extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate futures;
extern crate rusqlite;
extern crate tera;
extern crate lazy_static;

use lazy_static::lazy_static;
use tera::{Tera, compile_templates};
use hyper::{Response, StatusCode, Body};

use gotham::helpers::http::response::create_response;
use gotham::state::State;
// use gotham::handler::HandlerFuture;

use gotham::router::Router;
use gotham::router::builder::*;


lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = compile_templates!("templates/**/*");
        // and we can add more things to our instance if we want to
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/init-db").to(init_db);
    })
}

pub fn init_db(state: State) -> (State, Response<Body>) {
    let conn = rusqlite::Connection::open("database.sqlite3").unwrap();

    let result = conn.execute(
        "CREATE VIRTUAL TABLE pages USING fts4(title, keywords, body)",
        rusqlite::NO_PARAMS,
    );

    let response = match result {
        Ok(_) => String::from("table created successfully"),
        Err(err) => err.to_string()
    };

    let mut template_context = tera::Context::new();
    template_context.insert("message", &response);

    let contents = TERA.render("index.html", &template_context).unwrap();

    let res = create_response(
        &state, StatusCode::OK, mime::TEXT_HTML, contents
    );

    (state, res)
}


// pub fn say_hello(state: State) -> Box<HandlerFuture> {
//     let request = request_async(String::from("http://httpbin.org"));

//     Box::new(request.then(|result| {
//         let res = create_response(
//             &state, StatusCode::Ok, Some((result.unwrap(), mime::TEXT_PLAIN))
//         );

//         Ok((state, res))
//     }))
// }

pub fn main() {
    let addr = "127.0.0.1:8080";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(router()))
}
