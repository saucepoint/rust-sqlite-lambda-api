Created with [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda)

---

# Database Migration Guide

Because the SQLite file lives on AWS EFS, we *cannot* use wonderful tools like [dbmate](https://github.com/amacneil/dbmate).
We'll use the another
Lambda to manage changes to the database. On the bright side, you get to practice your Rust 😜

1. Define your migration as a *Rust function*. This can perform any arbitrary SQL command -- `CREATE TABLE, ALTER TABLE, DELETE TABLE, INSERT INTO, UPDATE, etc`
    ```Rust
    // Defined in db-migrations/migrations.rs

    fn create_user_table(connection: Connection) -> &'static str {
        connection.execute("
            CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT,
                address TEXT,
                UNIQUE (name)
            );
        ", ()).unwrap();

        "Created user table"
    }
    ```

2. Register your migration
    ```rust
    // Function already defined in db-migrations/migrations.rs
    // Just add to the existing match-case

    pub fn execute_migration(command: &str, connection: Connection) -> Result<&str, String> {
        match command {
            "drop_file" => Ok(drop_database()),
            "create_hello_table" => Ok(create_hello_table(connection)),
            // Register new migrations here
            "create_user_table" => Ok(create_user_table(connection)),
            _ => Err(format!("Unregistered Command: {}", command)),
        }
    }
    ```

2. Deploy the changes to AWS
    ```bash
    cargo lambda build --release && cargo lambda deploy --iam-role arn:aws:iam::<AWS_ACCOUNT_ID>:role/rust-sqlite-lambda-api --env-var MODE=prod
    ```

2. Apply the migration!
    ```
    cargo lambda invoke --remote --data-ascii '{"command": "create_user_table"}' db-migrations
    ```
