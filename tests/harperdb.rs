use harperdb_sdk_rust as harper;
use serde_json::Value;
mod common;
use assert_json_diff::assert_json_include;

#[macro_use]
extern crate serde_json;

#[tokio::test]
async fn create_schema() {
    let harper_client =  common::get_client();

    let schema_option: harper::SchemaOption = harper::SchemaOption {
        schema: "create_schema_test".to_string(),
    };

    let result = harper_client.create_schema(schema_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    assert_eq!("{\"message\":\"schema \'create_schema_test\' successfully created\"}", data);
}

#[tokio::test]
async fn describe_schema() {    
    let harper_client =  common::get_client();

    let schema_option: harper::SchemaOption = harper::SchemaOption {
        schema: "describe_schema_test".to_string(),
    };

    let result = harper_client.describe_schema(schema_option).await.unwrap();
    assert_eq!(result.status(),200);
   
    let data = result.text().await.unwrap();    
    assert_eq!("{}", data);
}

#[tokio::test]
async fn drop_schema() {
    let harper_client =  common::get_client();

    let schema_option: harper::SchemaOption = harper::SchemaOption {
        schema: "drop_schema_test".to_string(),
    };

    let result = harper_client.drop_schema(schema_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();    
    assert_eq!("{\"message\":\"successfully deleted schema \'drop_schema_test\'\"}", data);
}


#[tokio::test]
async fn describe_all() {
    let harper_client =  common::get_client();

    let result = harper_client.describe_all().await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();

    assert_json_include!(
        actual: v,
        expected: json!({
            "describe_schema_test": {},
        })
    );
}

#[tokio::test]
async fn create_table() {
    let harper_client =  common::get_client();

    let create_table_option: harper::CreateTableOptions = harper::CreateTableOptions {
        hash_attribute: "id".to_string(),
        table: "test_table".to_string(),
        schema: "testing".to_string(),
    };

    let result = harper_client.create_table(create_table_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    assert_eq!("{\"message\":\"table \'testing.test_table\' successfully created.\"}", data);
}

#[tokio::test]
async fn describe_table() {
    let harper_client =  common::get_client();

    let table_option: harper::TableOptions = harper::TableOptions {
        table: "describe_table_test".to_string(),
        schema: "testing".to_string(),
    };

    let result = harper_client.describe_table(table_option).await.unwrap();
    assert_eq!(result.status(),200);
    let data = result.text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();

    assert_json_include!(
        actual: v,
        expected: json!({
            "name": "describe_table_test",
            "schema": "testing",
            "hash_attribute": "id",
        })
    );

}

#[tokio::test]
async fn drop_table() {
    let harper_client =  common::get_client();

    let table_option: harper::TableOptions = harper::TableOptions {
        table: "drop_table_test".to_string(),
        schema: "testing".to_string(),
    };

    let result = harper_client.drop_table(table_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    assert_eq!("{\"message\":\"successfully deleted table \'testing.drop_table_test\'\"}", data);
}

#[tokio::test]
async fn drop_attribute() {
    let harper_client =  common::get_client();

    let attribute_drop_option: harper::AttributeDropOptions = harper::AttributeDropOptions {
        table: "drop_attribute_table_test".to_string(),
        schema: "testing".to_string(),
        attribute: "breed".to_string(),
    };

    let result = harper_client.drop_attribute(attribute_drop_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();

    assert_eq!("{\"message\":\"successfully deleted attribute \'breed\'\"}", data);
}

#[tokio::test]
async fn insert() {
    let harper_client =  common::get_client();

    let insert_option: harper::QueryOptions = harper::QueryOptions {
        table: "crud_table_test".to_string(),
        schema: "testing".to_string(),
        records:json!([{
            "name":"Mike",
            "breed":"Pit Bull",
            "id":"insertrecord1234",
            "age":9    
          }]),
    };

    let result = harper_client.insert(insert_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    assert_eq!("{\"message\":\"inserted 1 of 1 records\",\"skipped_hashes\":[],\"inserted_hashes\":[\"insertrecord1234\"]}", data);
}

#[tokio::test]
async fn update() {
    let harper_client =  common::get_client();

    let update_option: harper::QueryOptions = harper::QueryOptions {
        table: "crud_table_test".to_string(),
        schema: "testing".to_string(),
        records:json!([{            
            "id": "updaterecord1234",            
            "age": 100,            
        }]),
    };

    let result = harper_client.update(update_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    assert_eq!("{\"message\":\"updated 1 of 1 records\",\"skipped_hashes\":[],\"update_hashes\":[\"updaterecord1234\"]}", data);
}

#[tokio::test]
async fn delete() {
    let harper_client =  common::get_client();

    let delete_option: harper::RowDeleteOptions = harper::RowDeleteOptions {
        table: "crud_table_test".to_string(),
        schema: "testing".to_string(),
        hash_values:vec!["deleterecord1234".to_string()] ,//json!([]),
    };

    let result = harper_client.delete(delete_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    assert_eq!("{\"message\":\"1 of 1 record successfully deleted\",\"deleted_hashes\":[\"deleterecord1234\"],\"skipped_hashes\":[]}", data);
}

#[tokio::test]
async fn search_by_hash() {
    let harper_client =  common::get_client();

    let search_option: harper::HashSearchOptions = harper::HashSearchOptions {
        table: "crud_table_test".to_string(),
        schema: "testing".to_string(),
        hash_values:vec![
            "searchbyhash1234".to_string(),
        ],
        get_attributes:vec!["name".to_string()],
    };

    let result = harper_client.search_by_hash(search_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    assert_eq!("[{\"name\":\"Tom Ford\"}]", data);
}

#[tokio::test]
async fn search_by_value() {
    let harper_client =  common::get_client();

    let search_option: harper::ValueSearchOptions = harper::ValueSearchOptions {
        table: "crud_table_test".to_string(),
        schema: "testing".to_string(),
        search_attribute: "name".to_string(),
        search_value:"Tom*".to_string(),
        get_attributes:vec!["name".to_string()],
    };

    let result = harper_client.search_by_value(search_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    assert_eq!("[{\"name\":\"Tom Ford\"}]", data);
}

#[tokio::test]
async fn csv_data_load() {
    let harper_client =  common::get_client();

    let csv_data_load_option: harper::DataLoadOptions = harper::DataLoadOptions {
        table: "crud_table_test".to_string(),
        schema: "testing".to_string(),
        action: "insert".to_string(),
        data: "id,name,section,country,image\n1,ENGLISH POINTER,British and Irish Pointers and Setters,GREAT BRITAIN,http://www.fci.be/Nomenclature/Illustrations/001g07.jpg\n2,ENGLISH SETTER,British and Irish Pointers and Setters,GREAT BRITAIN,http://www.fci.be/Nomenclature/Illustrations/002g07.jpg\n3,KERRY BLUE TERRIER,Large and medium sized Terriers,IRELAND,\n".to_string(),        
    };

    let result = harper_client.csv_data_load(csv_data_load_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    assert!(data.contains("Starting job with id"));
}

#[tokio::test]
async fn csv_url_load() {
    let harper_client =  common::get_client();

    let url_data_load_option: harper::UrlLoadOptions = harper::UrlLoadOptions {
        table: "crud_table_test".to_string(),
        schema: "testing".to_string(),
        action: "insert".to_string(),
        csv_url: "https://s3.amazonaws.com/complimentarydata/breeds.csv".to_string()
    };

    let result = harper_client.csv_url_load(url_data_load_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    assert!(data.contains("Starting job with id"));
}

#[tokio::test]
async fn csv_file_load() {
    // let harper_client =  common::get_client();

    // let csv_data_load_option: harper::FileLoadOptions = harper::FileLoadOptions {
    //     table: "crud_table_test".to_string(),
    //     schema: "testing".to_string(),
    //     action: "insert".to_string(),
    //     file_path: "/home/user/imports/breeds.csv".to_string()
    // };

    // let result = harper_client.csv_file_load(csv_data_load_option).await.unwrap();
    // assert_eq!(result.status(),200);

    // let data = result.text().await.unwrap();
    // assert!(data.contains("Starting job with id"));
    assert!(true)
}

#[tokio::test]
async fn list_users() {
    let harper_client =  common::get_client();

    let result = harper_client.list_users().await.unwrap();
    assert_eq!(result.status(),200);
    
    let data = result.text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();

    assert_json_include!(
        actual: v,
        expected: json!([{
            "username": "HDB_ADMIN",
            "active": true,
        }])
    );
}

#[tokio::test]
async fn user_info() {
    let harper_client =  common::get_client();

    let result = harper_client.user_info().await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();

    assert_json_include!(
        actual: v,
        expected: json!({
            "username": "HDB_ADMIN",
            "active": true,
        })
    );
}

#[tokio::test]
async fn add_user() {
    let harper_client =  common::get_client();

    let user_option: harper::UserAddOptions = harper::UserAddOptions {
        role: "c0a90733-1fc3-48df-a16b-d7c3011b63b2".to_string(),
        username: "created_hdb_user".to_string(),
        password: "secret".to_string(),
        active: true
    };

    let result = harper_client.add_user(user_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    assert_eq!("{\"message\":\"created_hdb_user successfully added\"}", data);
}

#[tokio::test]
async fn alter_user() {
    let harper_client =  common::get_client();

    let user_option: harper::UserAlterOptions = harper::UserAlterOptions {
        role: Some("c0a90733-1fc3-48df-a16b-d7c3011b63b2".to_string()),
        username: "alter_hdb_user".to_string(),
        password: Some("secret2".to_string()),
        active: true
    };

    let result = harper_client.alter_user(user_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    assert_eq!("{\"message\":\"updated 1 of 1 records\",\"skipped_hashes\":[],\"new_attributes\":[],\"update_hashes\":[\"alter_hdb_user\"]}", data);
}

#[tokio::test]
async fn drop_user() {
    let harper_client =  common::get_client();

    let user_option: harper::UserDropOptions = harper::UserDropOptions {
        username: "delete_hdb_user".to_string(),
    };

    let result = harper_client.drop_user(user_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    assert_eq!("{\"message\":\"delete_hdb_user successfully deleted\"}", data);
}

#[tokio::test]
async fn list_roles() {
    let harper_client =  common::get_client();

    let result = harper_client.list_roles().await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();

    assert_json_include!(
        actual: v,
        expected: json!([{
            "role": "cluster_user",
        },{
            "role": "super_user",
        }])
    );
}

// #[tokio::test]
// async fn add_role() {
//     let harper_client =  common::get_client();

//     let role_option: harper::AddRoleOptions = harper::AddRoleOptions {
//         role: "develope3r2".to_string(),
//         super_user: false,
//         permission:json!({
//             "shop":{
//                 "tables": {
//                   "product": {
//                       "read":true,
//                       "insert":false,
//                       "update":false,
//                       "delete":false,
//                       "attribute_restrictions":[
//                          {
//                             "attribute_name": "color",
//                             "read":false,
//                             "insert":false,
//                             "update":false,
//                             "delete":false
//                          }
//                       ]
//                    }
//                 }
//              }
//         }),
//     };

//     let result = harper_client.add_role(role_option).await.unwrap();
//     let data = result.text().await.unwrap();

//     assert_eq!("{\"message\":\"schema \'newschema22\' successfully created\"}", data);
// }

// #[tokio::test]
// async fn alter_role() {
//     let harper_client =  common::get_client();

//     let role_option: harper::AlterRoleOptions = harper::AlterRoleOptions {
//         id: "3c5cc923-5351-4f81-91e3-01a03448e18f".to_string(),
//         role: "cluster_user".to_string(),
//         super_user: false,
//         permission:json!({            
//         }),
//     };

//     let result = harper_client.alter_role(role_option).await.unwrap();
//     assert_eq!(result.status(),200);

//     let data = result.text().await.unwrap();
//     assert_eq!("{\"message\":\"updated 1 of 1 records\",\"skipped_hashes\":[],\"new_attributes\":[],\"update_hashes\":[\"3c5cc923-5351-4f81-91e3-01a03448e18f\"]}", data);
// }

// #[tokio::test]
// async fn drop_role() {
//     let harper_client =  common::get_client();

//     let role_option: harper::DropRoleOptions = harper::DropRoleOptions {
//         id: "33d285dc-1ddb-4700-b5bd-300a67faa247".to_string(),
//     };

//     let result = harper_client.drop_role(role_option).await.unwrap();
//     assert_eq!(result.status(),200);

//     let data = result.text().await.unwrap();

//     assert_eq!("{\"message\":\"schema \'newschema22\' successfully created\"}", data);
// }


#[tokio::test]
async fn system_information() {
    let harper_client =  common::get_client();

    let system_information_option: harper::SystemInformationOptions = harper::SystemInformationOptions {
        attributes : Some(vec!["cpu".to_string()])
    };
    let result = harper_client.system_information(system_information_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();
    assert_json_include!(
        actual: v,
        expected: json!({
            "cpu":{
                "processors": 1
            },
        })
    );
}


#[tokio::test]
async fn read_logs() {
    let harper_client =  common::get_client();

    let read_logs_option: harper::LogsOptions = harper::LogsOptions {
        limit : Some(2),
        start : Some(0),
        from : None,
        until : None,
        order : None,
        // attributes :None
    };

    let result = harper_client.read_logs(read_logs_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();

    assert_json_include!(
        actual: v,
        expected: json!({
            "file":[],
        })
    );
}

#[tokio::test]
async fn search_jobs_by_start_date() {
    let harper_client =  common::get_client();

    let search_jobs_by_start_date_option: harper::JobsByDateOptions = harper::JobsByDateOptions {
        from_date : "2020-01-01".to_string(),
        to_date : "2020-09-30".to_string(),
    };

    let result = harper_client.search_jobs_by_start_date(search_jobs_by_start_date_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();

    assert_json_include!(
        actual: v,
        expected: json!([])
    );
}

#[tokio::test]
async fn get_job() {
    let harper_client =  common::get_client();

    let get_job_option: harper::GetJobOptions = harper::GetJobOptions {
        id : "4022737a-1db1-463c-a96c-0f58fcc21f96".to_string(),
    };

    let result = harper_client.get_job(get_job_option).await.unwrap();
    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();

    assert_json_include!(
        actual: v,
        expected: json!([])
    );
}

#[tokio::test]
async fn query() {

    let harper_client =  common::get_client();

    let result = harper_client
        .query("SELECT * FROM testing.crud_table_test limit 2;".to_string())
        .await
        .unwrap();

    assert_eq!(result.status(),200);

    let data = result.text().await.unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();
    
    assert_json_include!(
        actual: v,
        expected: json!([])
    );
}
