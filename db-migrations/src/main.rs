use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use dotenv::dotenv;
use std::env;
use rusqlite::Connection;

mod migrations;

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    command: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-runtime/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let command = event.payload.command.as_str();

    // set the sqlite database path based on the environment variable MODE
    let db_path = match env::var("MODE")?.as_str() {
        "local" => "./db.sqlite",
        "prod" => "/mnt/efs/db.sqlite",
        _ => panic!("MODE environment variable is not set"),
    };

    // creates/drops the database if it doesn't exist
    println!("Accessing DB at {}", db_path);
    if command == "drop_file"{
        fs::remove_file(db_path).unwrap();
    }
    else if Path::new(db_path).exists() {
        println!("db.sqlite exists");
    }
    else {
        fs::File::create(db_path).expect("Failed to create file");
    }
    let connection: Connection = Connection::open(db_path).unwrap();

    let result = migrations::execute_migration(command, connection).unwrap();

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Executed: {}.", result),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
