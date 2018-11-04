use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct Package {
    name: String,
    version: String,
    source: String,
    description: String
}

pub struct SqliteSearchProvider {
    connection: rusqlite::Connection
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

        let packages : Vec<Package> = stmt.query_map(
                &[&query_string],
                |row| Package{
                    name: row.get(0),
                    version: row.get(1),
                    source: row.get(2),
                    description: row.get(3)
                }
            )
            .unwrap()
            .map(|element| element.unwrap())
            .collect();

        packages
    }
}
