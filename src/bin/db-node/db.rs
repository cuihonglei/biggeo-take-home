use duckdb::{Connection, Result};

pub struct DB {}

pub fn duckdb(db_path: &str) -> Result<()> {
    #[derive(Debug)]
    struct Person {
        id: i32,
        name: String,
        data: Option<Vec<u8>>,
    }

    // Try to open or create the database at the specified path
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            eprintln!("Error opening the database: {}", e);
            return Err(e);
        }
    };

    // Create the `person` table if it doesn't exist
    if let Err(e) = conn.execute(
        "CREATE TABLE IF NOT EXISTS person (id INTEGER PRIMARY KEY, name TEXT, data BLOB)",
        [],
    ) {
        eprintln!("Error creating table: {}", e);
        return Err(e);
    }

    /*
    // Create a `Person` object
    let me = Person {
        id: 1, // Can be auto-generated or manually set
        name: String::from("John Doe"),
        data: Some(vec![1, 2, 3, 4]),
    };

    // Insert the person object into the database
    if let Err(e) = conn.execute(
        "INSERT INTO person (id, name, data) VALUES (?, ?, ?)",
        params![me.id, me.name, me.data],
    ) {
        eprintln!("Error inserting person: {}", e);
        return Err(e);
    }
     */

    // Prepare the SELECT statement to fetch the data
    let mut stmt = match conn.prepare("SELECT id, name, data FROM person") {
        Ok(statement) => statement,
        Err(e) => {
            eprintln!("Error preparing the SELECT statement: {}", e);
            return Err(e);
        }
    };

    // Query the data and map it to `Person` objects
    let person_iter = match stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    }) {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("Error querying data: {}", e);
            return Err(e);
        }
    };

    // Print the result for each person found
    for person in person_iter {
        match person {
            Ok(person) => {
                println!("Found person {:?}", person);
            }
            Err(e) => {
                eprintln!("Error processing person row: {}", e);
            }
        }
    }

    Ok(())
}
