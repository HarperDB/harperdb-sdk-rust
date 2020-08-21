# A Rust Client for HarperDB
## Introduction

HarperDB is a SQL/NoSQL data management platform. It is fully indexed, doesn't duplicate data, and runs on any device- from the edge to the cloud.

It is built natively as a set of micro-services, making development and integration easy and seamless. HarperDB utilizes a single-endpoint for all operations. HarperDB’s RESTful nature makes it stateless, stable, and scalable.


## Examples

```
[dependencies]
tokio = { version = "0.2", features = ["full"] }
serde_json = "1.0"
```
Basic usage:

```rust
use harperdb_sdk_rust::{ HarperConfig, Harper };
use harperdb_sdk_rust as harper;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use std::{error::Error};

#[macro_use]
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct DogRecord {
    id: usize,
    name: String,
    age: Option<usize>, 
    breed: Option<String>,
    image: Option<String>,
    __createdtime__: usize,
    __updatedtime__: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: HarperConfig = HarperConfig {
        url: "http://0.0.0.0:9925/",
        username: "HDB_ADMIN",
        password: "password",
        schema: "dev",
    };

    let harper_client = new(config);
    
    // Insert Record
    let insert_option: harper::QueryOptions = harper::QueryOptions {
        table: "dog",
        schema: "dev",
        records:json!([{
            "id": 1,
            "name": "Incredible Metal Chair",
            "breed": "Mutt",
            "age": 4,
            "image": "http://lorempixel.com/640/480/nature"
        }]),
    };
    let result = harper_client.insert(insert_option).await?;
    
    // Get Query Response
    println!("{:#?}", result.status());
    
    // Query Database
    let result = harper_client.query("SELECT * FROM dev.dog limit 2",).await?;
   
    let dog_record: Vec<DogRecord> = result.json().await?;
    println!("{:#?}", dog_record);

    // Get result as text
    // let data = result.text().await?;
    // println!("{:#?}", data);

}
```

## Test Environment Set-up

```
docker run -d -p 9925:9925 -v <Host Directory Path>:/opt/harperdb/hdb/ harperdb/hdb

./test_setup.sh

cargo test

./test_teardown.sh
```