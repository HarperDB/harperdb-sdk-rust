// #[macro_use]
// extern crate serde_json;

use harperdb_sdk_rust::{Harper, HarperConfig};


pub fn get_client() -> Harper {
    let config: HarperConfig = HarperConfig {
        url: "http://0.0.0.0:9925/".to_string(),
        username: "HDB_ADMIN".to_string(),
        password: "password".to_string(),
        schema: "shop".to_string(),
    };

    let harper_client = Harper::new(config);

    return harper_client;
}



// pub fn setup() {
//     let client = reqwest::Client::new();
    
//     let url = "http://0.0.0.0:9925/";
//     let username = "HDB_ADMIN";
//     let password = "password";
   

//     let map = json!({
//         "operation": "create_schema",
//         "schema": "describe_schema_test",
//     });
//     let _result = client
//             .post(url)
//             .basic_auth(username, Some(password))
//             .json(&map)
//             .send();

//     // let map = json!({
//     //     "operation": "create_schema",
//     //     "schema": "&options.role",
//     // });
//     // let result = client
//     //         .post(url)
//     //         .basic_auth(username, Some(password))
//     //         .json(&map)
//     //         .send();

//     //let data = result.text().unwrap();
//     // setup code specific to your library's tests would go here
// }

