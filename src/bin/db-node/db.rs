use duckdb::{Connection, Error as DuckDbError};

pub struct DB {
    conn: Connection,
}

pub fn run(db_path: &str) -> Result<DB, DuckDbError> {
    #[derive(Debug)]
    struct Person {
        id: i32,
        name: String,
        data: Option<Vec<u8>>,
    }

    // Try to open or create the database at the specified path
    let conn = Connection::open(db_path)?;

    let db = DB{conn};

    // Create the `person` table if it doesn't exist
    db.conn.execute(
        "CREATE TABLE IF NOT EXISTS person (id INTEGER PRIMARY KEY, name TEXT, data BLOB)",
        [],
    )?;

    // Uncomment and use this block to insert a sample `Person` if needed
    /*
    let me = Person {
        id: 1,
        name: String::from("John Doe"),
        data: Some(vec![1, 2, 3, 4]),
    };

    conn.execute(
        "INSERT INTO person (id, name, data) VALUES (?, ?, ?)",
        params![me.id, me.name, me.data],
    )?;
    */

    // Prepare the SELECT statement to fetch the data
    let mut stmt = db.conn.prepare("SELECT id, name, data FROM person")?;

    // Query the data and map it to `Person` objects
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    // Print the result for each person found
    for person in person_iter {
        match person {
            Ok(person) => println!("Found person {:?}", person),
            Err(e) => eprintln!("Error processing person row: {}", e),
        }
    }

    Ok(db)
}
