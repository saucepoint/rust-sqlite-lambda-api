use lambda_http::{Error, Request, RequestExt, Response, Body};
use serde::{Serialize, Deserialize};
use rusqlite::Connection;
use crate::database::hello::{HelloRecord, get_hello, create_hello};


// Utilized as both input requests (query params & request body)
#[derive(Debug, Serialize, Deserialize, Default)]
struct HelloRequest {
    #[serde(default)]
    name: String,
}


pub fn get(event: Request, connection: Connection) -> Result<Response<Body>, Error> {
    let args = event.query_string_parameters();

    // do stuff with query params, i.e. using as a filter when reading from the database
    // You should add input validation & logic for required vs. optional
    let name = args.first("name").unwrap_or("world").to_string();

    let record: HelloRecord = get_hello(connection, name).unwrap();
    Ok(record.into_response())
}


pub fn post(event: Request, connection: Connection) -> Result<Response<Body>, Error> {
    let args: HelloRequest = event.payload().unwrap_or_else(|_parse_error| None).unwrap_or_default();

    // do stuff with request body, such as upserting into the database
    let record: HelloRecord = create_hello(connection, args.name).unwrap();
    Ok(record.into_response())
}
