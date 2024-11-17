use duckdb::{params, Connection, Result};
use std::env;
use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Get the address and database path from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <address> <db_path>", args[0]);
        std::process::exit(1);
    }

    let address = &args[1];
    let db_path = &args[2];

    println!("DB node listening on {}", address);
    println!("Using database at {}", db_path);

    // DuckDB - Pass the db_path from command line
    let _ = duckdb(db_path);

    // API
    let listener = TcpListener::bind(address).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();

            // TODO Handle the DB operations: insert, analyse
            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}

fn duckdb(db_path: &str) -> Result<()> {
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
