//! A Hello World example application for working with Gotham.

extern crate gotham;
extern crate gotham_derive;

extern crate hyper;
extern crate mime;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate futures;
extern crate rusqlite;
extern crate tera;
extern crate lazy_static;

mod sqlite;

// ------------------------------------

use lazy_static::lazy_static;
use tera::{Tera, compile_templates};
use hyper::{Response, StatusCode, Body};

use gotham::helpers::http::response::create_response;
use gotham::state::{FromState, State};

use gotham::router::Router;
use gotham::router::builder::*;

use gotham_derive::StaticResponseExtender;
use gotham_derive::StateData;
use serde_derive::{Serialize, Deserialize};

// ------------------------------------

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct QueryStringExtractor {
    query: Option<String>,
}

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = compile_templates!("templates/**/*");
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };

    pub static ref DATABASE_FILE: String = {
        match std::env::var("DATABASE_FILE") {
            Ok(filename) => filename,
            Err(_) => String::from("database.sqlite3")
        }
    };
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/")
            .with_query_string_extractor::<QueryStringExtractor>()
            .to(search);
    })
}

pub fn search(mut state: State) -> (State, Response<Body>) {
    let search_provider = sqlite::SqliteSearchProvider::new(&DATABASE_FILE);

    let mut template_context = tera::Context::new();

    let query_param = QueryStringExtractor::take_from(&mut state);

    if let Some(query_string) = query_param.query {
        let packages = search_provider.search_packages(&query_string);

        template_context.insert("package_results", &packages);
        template_context.insert("query", &query_string);
    } else {
        template_context.insert("query", &"".to_string());
    }

    let contents = TERA.render("index.html", &template_context).unwrap();

    let res = create_response(
        &state, StatusCode::OK, mime::TEXT_HTML, contents
    );

    (state, res)
}

pub fn main() {
    let addr = "0.0.0.0:8080";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(router()))
}
