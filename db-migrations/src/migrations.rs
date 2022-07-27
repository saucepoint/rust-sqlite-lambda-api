

pub fn execute_migration(command: &str, connection: Connection) -> Result<&str, String> {
    match command {
        "drop_file" => Ok(drop_database()),
        "create_hello_table" => Ok(create_hello_table(connection)),
        _ => Err(format!("Unregistered Command: {}", command)),
    }
}

fn drop_database() -> &'static str {
    // dropping of the database is managed by main.rs
    // which will remove the file if it exists
    "Dropped database"
}

fn create_hello_table(connection: Connection) -> &'static str {
    connection.execute("
        CREATE TABLE IF NOT EXISTS hello (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            count INTEGER,
            UNIQUE (name)
        );
    ", ()).unwrap();

    "Created hello table"
}
