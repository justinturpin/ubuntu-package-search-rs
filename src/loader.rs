//! Logic for loading the Ubuntu package data into SQLite's full text search tables.

use std::io::{BufRead, BufReader};
use std::io::Read;
use super::sqlite::Package;


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
