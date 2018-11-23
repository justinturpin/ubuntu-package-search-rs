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
extern crate flate2;

mod sqlite;
mod loader;

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

#[derive(Serialize, Deserialize)]
enum SearchType {
    #[serde(rename = "package")]
    Package,
    #[serde(rename = "file")]
    File
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct QueryStringExtractor {
    query: Option<String>,

    #[serde(rename = "type")]
    search_type: Option<SearchType>
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
        route.get("/search")
            .with_query_string_extractor::<QueryStringExtractor>()
            .to(search);

        route.get("/")
            .to(index);

        route.get("/about.html")
            .to(about);
    })
}

/// Handler for the main Index page, which is just the search minus the actual results.
/// This can probably be combined with search at some point?
pub fn index(state: State) -> (State, Response<Body>) {
    let mut template_context = tera::Context::new();

    template_context.insert("query", &"".to_string());
    template_context.insert("search_type", &SearchType::Package);

    let contents = TERA.render("index.html", &template_context).unwrap();

    let res = create_response(
        &state, StatusCode::OK, mime::TEXT_HTML, contents
    );

    (state, res)
}

/// Handler the Search page
pub fn search(mut state: State) -> (State, Response<Body>) {
    let search_provider = sqlite::SqliteSearchProvider::new(&DATABASE_FILE);

    let mut template_context = tera::Context::new();

    let query_param = QueryStringExtractor::take_from(&mut state);

    // By default, set the search type to Package

    let search_type = match query_param.search_type {
        Some(s) => s,
        None => SearchType::Package
    };

    template_context.insert("search_type", &search_type);

    if let Some(query_string) = query_param.query  {
        template_context.insert("query", &query_string);

        match search_type {
            SearchType::Package => {
                let packages = search_provider.search_packages(&query_string);

                template_context.insert("package_results", &packages);
            },
            SearchType::File => {
                let files = search_provider.search_files(&query_string);

                template_context.insert("file_results", &files);
            }
        }
    } else {
        template_context.insert("query", &"".to_string());
    }

    let contents = TERA.render("index.html", &template_context).unwrap();

    let res = create_response(
        &state, StatusCode::OK, mime::TEXT_HTML, contents
    );

    (state, res)
}

/// Handler for the About page
pub fn about(state: State) -> (State, Response<Body>) {
    let contents = TERA.render("about.html", &tera::Context::default()).unwrap();

    let res = create_response(
        &state, StatusCode::OK, mime::TEXT_HTML, contents
    );

    (state, res)
}

pub fn main() {
    // Load data into our SQLite database file

    let thread = std::thread::spawn(move || {
        let path_string = DATABASE_FILE.to_string();

        let path = std::path::Path::new(&path_string);

        if path.exists() {
            println!("Database file {} already exists, not loading.", path_string);
        } else {
            println!("Loading data into sqlite database {}...", path_string);

            loader::load_data(&DATABASE_FILE);
        }
    });

    // Start server
    let addr = "0.0.0.0:8080";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(router()));

    thread.join().unwrap();
}
