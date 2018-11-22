//! Sqlite abstraction stuff

use serde_derive::Serialize;
use std::io::{BufRead, BufReader};
use std::io::Read;

#[derive(Debug, Serialize)]
pub struct Package {
    name: String,
    version: String,
    source: String,
    description: String
}

#[derive(Debug, Serialize)]
pub struct PackageFile {
    filename: String,
    package: String
}

pub struct SqliteSearchProvider {
    connection: rusqlite::Connection
}

impl Package {
    fn empty() -> Package {
        Package {
            name: String::new(),
            version: String::new(),
            source: String::new(),
            description: String::new()
        }
    }
}

impl SqliteSearchProvider {
    /// Construct a new SqliteSearchProvider given a database filename
    pub fn new(database_filename: &str) -> SqliteSearchProvider {
        SqliteSearchProvider {
            connection: rusqlite::Connection::open(database_filename).unwrap()
        }
    }

    /// Search for packages that match a given query string
    pub fn search_packages(&self, query_string: &String) -> Vec<Package> {
        let mut stmt = self.connection
            .prepare("SELECT name, version, source, description FROM packages WHERE name MATCH ?1 LIMIT 80")
            .unwrap();

        let package_query = stmt.query_map(
                &[&query_string],
                |row| Package{
                    name: row.get(0),
                    version: row.get(1),
                    source: row.get(2),
                    description: row.get(3)
                }
            );

        // If the package query is okay, return the list of packages, otherwise just
        // return an empty list to ease with error scenarios. Eventually this will be
        // a proper error result.

        match package_query {
            Ok(package_iter) => package_iter.map(
                    |element| element.unwrap()
                ).collect(),
            Err(_) => Vec::new()
        }
    }

    /// Search for files that match a given query string
    pub fn search_files(&self, query_string: &String) -> Vec<PackageFile> {
        let mut stmt = self.connection
            .prepare("SELECT filename, package FROM contents WHERE filename MATCH ?1 LIMIT 80")
            .unwrap();

        let file_query = stmt.query_map(
                &[&query_string],
                |row| PackageFile{
                    filename: row.get(0),
                    package: row.get(1)
                }
            );

        match file_query {
            Ok(file_iter) => file_iter.map(
                    |element| element.unwrap()
                ).collect(),
            Err(_) => Vec::new()
        }
    }
}

struct PackageReader<T> {
    source: String,
    reader: BufReader<T>
}

impl<T> PackageReader<T> {
    fn new(source: String, reader: BufReader<T>) -> PackageReader<T> {
        PackageReader{
            source: source,
            reader: reader
        }
    }
}

impl<T> Iterator for PackageReader<T> where T: Read {
    type Item = Package;

    fn next(&mut self) -> Option<Package> {
        let mut current_package = Package::empty();
        current_package.source = self.source.clone();

        let mut name_set = false;

        for line in self.reader.by_ref().lines() {
            let line = line.unwrap();
            let line = line.trim();

            let line_split: Vec<&str> = line.split(":").collect();

            if line_split.len() == 2 {
                let (key, value) = (line_split[0], line_split[1]);

                match key {
                    "Package" => { name_set = true; current_package.name = String::from(value) },
                    "Version" => { current_package.version = String::from(value) },
                    "Description" => { current_package.description = String::from(value) },
                    _ => ()
                }
            }

            if line == "" && name_set {
                return Some(current_package)
            }
        }

        None
    }
}


/// Loads data from Ubuntu and stores it into SQLite
pub fn load_data(sqlite_database: &str) {
    let mut connection = rusqlite::Connection::open(sqlite_database).unwrap();

    connection.execute(
        "CREATE VIRTUAL TABLE packages USING fts4(name, version, source, description)", rusqlite::NO_PARAMS
    ).unwrap();

    let source_list = vec![
        ("main", "http://archive.ubuntu.com/ubuntu/dists/bionic/main/binary-amd64/Packages.gz"),
        ("universe", "http://archive.ubuntu.com/ubuntu/dists/bionic/universe/binary-amd64/Packages.gz"),
        ("multiverse", "http://archive.ubuntu.com/ubuntu/dists/bionic/multiverse/binary-amd64/Packages.gz"),
        ("restricted", "http://archive.ubuntu.com/ubuntu/dists/bionic/restricted/binary-amd64/Packages.gz"),
    ];

    // Set up a Sqlite transaction to make this insert a whole lot faster
    let transaction = connection.transaction().unwrap();

    {
        for (source, url) in source_list.iter() {
            let mut packages_stmt = transaction
                .prepare("INSERT INTO packages (name, version, source, description) VALUES (?, ?, ?, ?)")
                .unwrap();

            let response = reqwest::get(*url)
                .unwrap();

            let reader = BufReader::new(flate2::read::GzDecoder::new(response));

            for package in PackageReader::new(String::from(*source), reader) {
                packages_stmt.execute(
                        &[&package.name, &package.version, &package.source, &package.description]
                    ).unwrap();
            }
        }
    }

    transaction.commit().unwrap();
}
