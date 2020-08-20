use harperdb_sdk_rust::{ HarperConfig, Harper };
use harperdb_sdk_rust as harper;
use serde_json::{Value};
use std::{error::Error};

#[macro_use]
extern crate serde_json;

//docker run -d -p 9925:9925 -v /home/dallen/Codes/harperdbdata/:/opt/harperdb/hdb/ harperdb/hdb

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: HarperConfig = HarperConfig {
        url: "http://0.0.0.0:9925/".to_string(),
        username: "HDB_ADMIN".to_string(),
        password: "password".to_string(),
        schema: "shop".to_string(),
    };

    let harper_client = Harper::new(config);


    // Describe Table ------------------------------------------
    // let table_option: harper::TableOptions = harper::TableOptions {
    //     table: "test_table".to_string(),
    //     schema: "describe_schema_test".to_string(),
    // };
    // let result = harper_client.describe_table(table_option).await?;


    // // Create Schema a ------------------------------------------
    // let schema_option: harper::SchemaOption = harper::SchemaOption {
    //     schema: "newschema2".to_string(),
    // };
    // let result = harper_client.create_schema(schema_option).await?;

    // // Delete Schema ------------------------------------------
    // let schema_option: harper::SchemaOption = harper::SchemaOption {
    //     schema: "newschema22".to_string(),
    // };
    // let result = harper_client.drop_schema(schema_option).await?;

    // Describe Schema ------------------------------------------
    // let schema_option: harper::SchemaOption = harper::SchemaOption {
    //     schema: "describe_schema_test".to_string(),
    // };
    // let result = harper_client.describe_schema(schema_option).await?;

    // // Describe All ------------------------------------------
    // let result = harper_client.describe_all().await?;


    // Create Table ------------------------------------------
    // let create_table_option: harper::CreateTableOptions = harper::CreateTableOptions {
    //     hash_attribute: "id".to_string(),
    //     table: "test_table".to_string(),
    //     schema: "describe_schema_test".to_string(),
    // };
    // let result = harper_client.create_table(create_table_option).await?;


    // // Drop Table  ------------------------------------------
    // let table_option: harper::TableOptions = harper::TableOptions {
    //     table: "test2".to_string(),
    //     schema: "newschema".to_string(),
    // };
    // let result = harper_client.drop_table(table_option).await?;


    // Drop Table Attribute ------------------------------------------
    // let attribute_drop_option: harper::AttributeDropOptions = harper::AttributeDropOptions {
    //     table: "drop_attribute_table_test".to_string(),
    //     schema: "testing".to_string(),
    //     attribute: "breed".to_string(),
    // };
    // let result = harper_client.drop_attribute(attribute_drop_option).await?;

    // Insert Records  ------------------------------------------
    // let insert_option: harper::QueryOptions = harper::QueryOptions {
    //     table: "crud_table_test".to_string(),
    //     schema: "testing".to_string(),
    //     records:json!([{
    //         "id": "record1234",
    //         "name": "Incredible Metal Chair",
    //         "modelNumber": 356625,
    //         "unitPrice": "$1700.00",
    //         "color": "#961",
    //         "material": "Rubber",
    //         "isDiscontinued": true,
    //         "image": "http://lorempixel.com/640/480/nature"
    //     }]),
    // };
    // let result = harper_client.insert(insert_option).await?;

    // Update Records ------------------------------------------
    // let update_option: harper::QueryOptions = harper::QueryOptions {
        // table: "crud_table_test".to_string(),
        // schema: "testing".to_string(),
    //     records:json!([{            
    //         "id": "record1234",           
    //         "color": "red",            
    //     }]),
    // };
    // let result = harper_client.update(update_option).await?;

    // Delete Records ------------------------------------------
    // let delete_option: harper::RowDeleteOptions = harper::RowDeleteOptions {
    //     table: "crud_table_test".to_string(),
    //     schema: "testing".to_string(),
    //     hash_values:vec!["record1234".to_string()] ,//json!([]),
    // };
    // let result = harper_client.delete(delete_option).await?;


    // Search By Hash------------------------------------------
    // let search_option: harper::HashSearchOptions = harper::HashSearchOptions {
        // table: "crud_table_test".to_string(),
        // schema: "testing".to_string(),
    //     hash_values:vec![
    //         "updaterecord1234".to_string(),
    //     ],
    //     get_attributes:vec!["name".to_string()],
    // };

    // let result = harper_client.search_by_hash(search_option).await?;

    // Search By Value------------------------------------------
    // let search_option: harper::ValueSearchOptions = harper::ValueSearchOptions {
        // table: "crud_table_test".to_string(),
        // schema: "testing".to_string(),
    //     search_attribute: "name".to_string(),
    //     search_value:"Tom*".to_string(),
    //     get_attributes:vec!["name".to_string()],
    // };

    // let result = harper_client.search_by_value(search_option).await?;

    // CSV Data Load ------------------------------------------
    // let csv_data_load_option: harper::DataLoadOptions = harper::DataLoadOptions {
    //     table: "crud_table_test".to_string(),
    //     schema: "testing".to_string(),
    //     action: "insert".to_string(),
    //     data: "id,name,section,country,image\n1,ENGLISH POINTER,British and Irish Pointers and Setters,GREAT BRITAIN,http://www.fci.be/Nomenclature/Illustrations/001g07.jpg\n2,ENGLISH SETTER,British and Irish Pointers and Setters,GREAT BRITAIN,http://www.fci.be/Nomenclature/Illustrations/002g07.jpg\n3,KERRY BLUE TERRIER,Large and medium sized Terriers,IRELAND,\n".to_string(),        
    // };

    // let result = harper_client.csv_data_load(csv_data_load_option).await?;

    // CSV URL Load ------------------------------------------
    // let csv_data_load_option: harper::FileLoadOptions = harper::FileLoadOptions {
    //     table: "crud_table_test".to_string(),
    //     schema: "testing".to_string(),
    //     action: "insert".to_string(),
    //     file_path: "~/Codes/harperdb-sdk-rust/breeds.csv".to_string()
    // };

    // let result = harper_client.csv_file_load(csv_data_load_option).await?;

    // // List Users ------------------------------------------    
    // let result = harper_client.list_users().await?;

    // // User Info ------------------------------------------    
    // let result = harper_client.user_info().await?;

    
    // Add User  ------------------------------------------    
    // let user_option: harper::UserAddOptions = harper::UserAddOptions {
    //     role: "c0a90733-1fc3-48df-a16b-d7c3011b63b2".to_string(),
    //     username: "john".to_string(),
    //     password: "secret".to_string(),
    //     active: true
    // };
    // let result = harper_client.add_user(user_option).await?;

    // Alter User  ------------------------------------------    
     let user_option: harper::UserAlterOptions = harper::UserAlterOptions {
        role: Some("c0a90733-1fc3-48df-a16b-d7c3011b63b2".to_string()),
        username: "john".to_string(),
        password: Some("secret2".to_string()),
        active: true
    };
    let result = harper_client.alter_user(user_option).await?;

    // Drop User  ------------------------------------------    
    // let user_option: harper::UserDropOptions = harper::UserDropOptions {
    //     username: "john".to_string(),
    // };
    // let result = harper_client.drop_user(user_option).await?;


    // // List Roles ------------------------------------------    
    // let result = harper_client.list_roles().await?;


    // Add Role ------------------------------------------    
    // let role_option: harper::AddRoleOptions = harper::AddRoleOptions {
    //     role: "develope3r".to_string(),
    //     super_user: false,
    //     permission:json!({
    //         // "testing":{
    //         //     "tables": {
    //         //       "crud_table_test": {
    //         //           "read":true,
    //         //           "insert":false,
    //         //           "update":false,
    //         //           "delete":false,
    //         //         //   "attribute_restrictions":[
    //         //         //      {
    //         //         //         "attribute_name": "color",
    //         //         //         "read":false,
    //         //         //         "insert":false,
    //         //         //         "update":false,
    //         //         //         "delete":false
    //         //         //      }
    //         //         //   ]
    //         //        }
    //         //     }
    //         //  }
    //     }),
    // };

    // let result = harper_client.add_role(role_option).await?;


    // // Alter Role ------------------------------------------    
    // let role_option: harper::AlterRoleOptions = harper::AlterRoleOptions {
    //     id: "3c5cc923-5351-4f81-91e3-01a03448e18f".to_string(),
    //     role: "cluster_user".to_string(),
    //     super_user: false,
    //     permission:json!({
    //         // "shop":{
    //         //     "tables": {
    //         //       "product": {
    //         //           "read":false,
    //         //           "insert":true,
    //         //           "update":true,
    //         //           "delete":true,
    //         //           "attribute_restrictions":[
    //         //              {
    //         //                 "attribute_name": "color",
    //         //                 "read":false,
    //         //                 "insert":true,
    //         //                 "update":true,
    //         //                 "delete":false
    //         //              }
    //         //           ]
    //         //        }
    //         //     }
    //         //  }
    //     }),
    // };
    // let result = harper_client.alter_role(role_option).await?;

    // // Drop Role ------------------------------------------    
    // let role_option: harper::DropRoleOptions = harper::DropRoleOptions {
    //     id: "33d285dc-1ddb-4700-b5bd-300a67faa247".to_string(),
    // };
    // let result = harper_client.drop_role(role_option).await?;

    // // System Information ------------------------------------------    
    // let system_information_option: harper::SystemInformationOptions = harper::SystemInformationOptions {
    //     attributes : Some(vec!["cpu".to_string()])
    //     // attributes :None
    // };
    // let result = harper_client.system_information(system_information_option).await?;
   
    // // Delete Files Before------------------------------------------!!!!!!    
    // // Export To S3 ------------------------------------------!!!!!!  
    // // Export To Local ------------------------------------------!!!!!!  

    // Read Log ------------------------------------------    
    // let read_logs_option: harper::LogsOptions = harper::LogsOptions {
    //     limit : Some(2),
    //     start : Some(0),
    //     from : None,
    //     until : None,
    //     order : None,
    //     // attributes :None
    // };
    // let result = harper_client.read_logs(read_logs_option).await?;

    // Search Jobs By Start Date ------------------------------------------    
    // let search_jobs_by_start_date_option: harper::JobsByDateOptions = harper::JobsByDateOptions {
    //     from_date : "2019-01-01".to_string(),
    //     to_date : "2020-12-30".to_string(),
    // };

    // let result = harper_client.search_jobs_by_start_date(search_jobs_by_start_date_option).await?;

    // Get Job ------------------------------------------    
    // let get_job_option: harper::GetJobOptions = harper::GetJobOptions {
    //     id : "d8b70ed4-a62a-45ef-bf86-15508c4ba10a".to_string(),
    // };

    // let result = harper_client.get_job(get_job_option).await?;

    // SQL Query ------------------------------------------    
    // let result = harper_client.query("SELECT * FROM testing.crud_table_test limit 2".to_string(),).await?;

    // println!("{:#?}", result.status());
    
    let data = result.text().await?;
    let v: Value = serde_json::from_str(&data).unwrap();
    
    
    println!("{:#?}", data);
    println!("{:#?}", data.contains("Starting job with id"));
    println!("{:#?}", v);

    Ok(())
}
