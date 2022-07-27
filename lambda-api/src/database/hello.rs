use lambda_http::{Body, Response};
use std::{fmt::Error};
use serde::{Serialize, Deserialize};
use rusqlite::Connection;


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HelloRecord {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub count: i64
}


// To use database records as response bodies, you should implement 
// `into_response(self) -> Response<Body>`
impl HelloRecord {
    pub fn into_response(self) -> Response<Body> {
        let s = serde_json::to_string_pretty(&self).unwrap();
        let resp = Body::from(s);
        let resp = Response::builder()
            .status(200)
            .header("content-type", "text/html")
            .body(resp)
            .map_err(Box::new).unwrap();
        resp
    }
}


pub fn get_hello(connection: Connection, name: String) -> Result<HelloRecord, Error> {
    let mut stmt = connection.prepare("SELECT id, name, count FROM hello WHERE name = ?").unwrap();
    let mut rows = stmt.query_map(&[&name], |row| {
        Ok(HelloRecord {
            id: row.get(0)?,
            name: row.get(1)?,
            count: row.get(2)?
        })
    }).unwrap();

    let result: HelloRecord = rows.next().unwrap().unwrap();
    Ok(result)
}


pub fn create_hello(connection: Connection, name: String) -> Result<HelloRecord, Error> {
    // do stuff with request body, such as upserting into the database
    let statement ="
        INSERT INTO hello (name, count)
        VALUES (?, 1)
        ON CONFLICT (name)
        DO UPDATE SET count = count + 1;
    ";
    connection.execute(statement, &[&name]).unwrap();

    let record: HelloRecord = get_hello(connection, name).unwrap();
    Ok(record)
}
