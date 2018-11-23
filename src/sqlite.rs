//! Sqlite abstraction stuff

use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub source: String,
    pub description: String
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
    pub fn empty() -> Package {
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
