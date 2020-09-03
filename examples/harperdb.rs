use harperdb_sdk_rust::{ HarperConfig, Harper };
use harperdb_sdk_rust as harper;
use serde_json::{Value};
use std::{error::Error};

#[macro_use]
extern crate serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: HarperConfig = HarperConfig {
        url: "http://0.0.0.0:9925/",
        username: "HDB_ADMIN",
        password: "password",
        schema: "shop",
    };

    let harper_client = Harper::new(config);


    // Describe Table ------------------------------------------
    let table_option: harper::TableOptions = harper::TableOptions {
        table: "test_table",
        schema: "describe_schema_test",
    };
    let result = harper_client.describe_table(table_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Create Schema ------------------------------------------
    let schema_option: harper::SchemaOption = harper::SchemaOption {
        schema: "newschema2",
    };
    let result = harper_client.create_schema(schema_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Delete Schema ------------------------------------------
    let schema_option: harper::SchemaOption = harper::SchemaOption {
        schema: "newschema22",
    };
    let result = harper_client.drop_schema(schema_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    //Describe Schema ------------------------------------------
    let schema_option: harper::SchemaOption = harper::SchemaOption {
        schema: "describe_schema_test",
    };
    let result = harper_client.describe_schema(schema_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Describe All ------------------------------------------
    let result = harper_client.describe_all().await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Create Table ------------------------------------------
    let create_table_option: harper::CreateTableOptions = harper::CreateTableOptions {
        hash_attribute: "id",
        table: "test_table",
        schema: "describe_schema_test",
    };
    let result = harper_client.create_table(create_table_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Drop Table  ------------------------------------------
    let table_option: harper::TableOptions = harper::TableOptions {
        table: "test2",
        schema: "newschema",
    };
    let result = harper_client.drop_table(table_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Drop Table Attribute ------------------------------------------
    let attribute_drop_option: harper::AttributeDropOptions = harper::AttributeDropOptions {
        table: "drop_attribute_table_test",
        schema: "testing",
        attribute: "breed",
    };
    let result = harper_client.drop_attribute(attribute_drop_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Insert Records  ------------------------------------------
    let insert_option: harper::QueryOptions = harper::QueryOptions {
        table: "crud_table_test",
        schema: "testing",
        records:json!([{
            "id": "record1234",
            "name": "Incredible Metal Chair",
            "modelNumber": 356625,
            "unitPrice": "$1700.00",
            "color": "#961",
            "material": "Rubber",
            "isDiscontinued": true,
            "image": "http://lorempixel.com/640/480/nature"
        }]),
    };
    let result = harper_client.insert(insert_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Update Records ------------------------------------------
    let update_option: harper::QueryOptions = harper::QueryOptions {
        table: "crud_table_test",
        schema: "testing",
        records:json!([{            
            "id": "record1234",           
            "color": "red",            
        }]),
    };
    let result = harper_client.update(update_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Delete Records ------------------------------------------
    let delete_option: harper::RowDeleteOptions = harper::RowDeleteOptions {
        table: "crud_table_test",
        schema: "testing",
        hash_values:vec!["record1234"] ,//json!([]),
    };
    let result = harper_client.delete(delete_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);


    // Search By Hash------------------------------------------
    let search_option: harper::HashSearchOptions = harper::HashSearchOptions {
        table: "crud_table_test",
        schema: "testing",
        hash_values:vec![
            "updaterecord1234",
        ],
        get_attributes:vec!["name"],
    };

    let result = harper_client.search_by_hash(search_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Search By Value------------------------------------------
    let search_option: harper::ValueSearchOptions = harper::ValueSearchOptions {
        table: "crud_table_test",
        schema: "testing",
        search_attribute: "name",
        search_value:"Tom*",
        get_attributes:vec!["name"],
    };

    let result = harper_client.search_by_value(search_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // CSV Data Load ------------------------------------------
    let csv_data_load_option: harper::DataLoadOptions = harper::DataLoadOptions {
        table: "crud_table_test",
        schema: "testing",
        action: "insert",
        data: "id,name,section,country,image\n1,ENGLISH POINTER,British and Irish Pointers and Setters,GREAT BRITAIN,http://www.fci.be/Nomenclature/Illustrations/001g07.jpg\n2,ENGLISH SETTER,British and Irish Pointers and Setters,GREAT BRITAIN,http://www.fci.be/Nomenclature/Illustrations/002g07.jpg\n3,KERRY BLUE TERRIER,Large and medium sized Terriers,IRELAND,\n",        
    };

    let result = harper_client.csv_data_load(csv_data_load_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);


    // CSV URL Load ------------------------------------------
    let csv_data_load_option: harper::FileLoadOptions = harper::FileLoadOptions {
        table: "crud_table_test",
        schema: "testing",
        action: "insert",
        file_path: "~/Codes/harperdb-sdk-rust/breeds.csv"
    };

    let result = harper_client.csv_file_load(csv_data_load_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // List Users ------------------------------------------    
    let result = harper_client.list_users().await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // User Info ------------------------------------------    
    let result = harper_client.user_info().await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);
    
    // Add User  ------------------------------------------    
    let user_option: harper::UserAddOptions = harper::UserAddOptions {
        role: "c0a90733-1fc3-48df-a16b-d7c3011b63b2",
        username: "john",
        password: "secret",
        active: true
    };
    let result = harper_client.add_user(user_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Alter User  ------------------------------------------    
     let user_option: harper::UserAlterOptions = harper::UserAlterOptions {
        role: Some("c0a90733-1fc3-48df-a16b-d7c3011b63b2"),
        username: "john",
        password: Some("secret2"),
        active: true
    };
    let result = harper_client.alter_user(user_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Drop User  ------------------------------------------    
    let user_option: harper::UserDropOptions = harper::UserDropOptions {
        username: "john",
    };
    let result = harper_client.drop_user(user_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // List Roles ------------------------------------------    
    let result = harper_client.list_roles().await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Add Role ------------------------------------------    
    let role_option: harper::AddRoleOptions = harper::AddRoleOptions {
        role: "develope3r",
        super_user: false,
        permission:json!({
            "testing":{
                "tables": {
                  "crud_table_test": {
                      "read":true,
                      "insert":false,
                      "update":false,
                      "delete":false,
                      "attribute_restrictions":[
                         {
                            "attribute_name": "color",
                            "read":false,
                            "insert":false,
                            "update":false,
                            "delete":false
                         }
                      ]
                   }
                }
             }
        }),
    };

    let result = harper_client.add_role(role_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Alter Role ------------------------------------------    
    let role_option: harper::AlterRoleOptions = harper::AlterRoleOptions {
        id: "3c5cc923-5351-4f81-91e3-01a03448e18f",
        role: "cluster_user",
        super_user: false,
        permission:json!({
            "shop":{
                "tables": {
                  "product": {
                      "read":false,
                      "insert":true,
                      "update":true,
                      "delete":true,
                      "attribute_restrictions":[
                         {
                            "attribute_name": "color",
                            "read":false,
                            "insert":true,
                            "update":true,
                            "delete":false
                         }
                      ]
                   }
                }
             }
        }),
    };
    let result = harper_client.alter_role(role_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Drop Role ------------------------------------------    
    let role_option: harper::DropRoleOptions = harper::DropRoleOptions {
        id: "33d285dc-1ddb-4700-b5bd-300a67faa247",
    };
    let result = harper_client.drop_role(role_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // System Information ------------------------------------------    
    let system_information_option: harper::SystemInformationOptions = harper::SystemInformationOptions {
        attributes : Some(vec!["cpu"])
        // attributes :None
    };
    let result = harper_client.system_information(system_information_option).await?;
   
    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);
    // Delete Files Before------------------------------------------!!!!!!    
    // Export To S3 ------------------------------------------!!!!!!  
    // Export To Local ------------------------------------------!!!!!!  

    // Read Log ------------------------------------------    
    let read_logs_option: harper::LogsOptions = harper::LogsOptions {
        limit : Some(2),
        start : Some(0),
        from : None,
        until : None,
        order : None,
        // attributes :None
    };
    let result = harper_client.read_logs(read_logs_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Search Jobs By Start Date ------------------------------------------    
    let search_jobs_by_start_date_option: harper::JobsByDateOptions = harper::JobsByDateOptions {
        from_date : "2019-01-01",
        to_date : "2020-12-30",
    };

    let result = harper_client.search_jobs_by_start_date(search_jobs_by_start_date_option).await?;

    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    // Get Job ------------------------------------------    
    let get_job_option: harper::GetJobOptions = harper::GetJobOptions {
        id : "d8b70ed4-a62a-45ef-bf86-15508c4ba10a",
    };

    let result = harper_client.get_job(get_job_option).await?;


    println!("{:#?}", result.status());
    let v: Value =  result.json().await?;    
    //let data = result.text().await?;
    println!("{:#?}", v);

    //SQL Query ------------------------------------------  
          
    let result = harper_client.query("SELECT * FROM testing.crud_table_test WHERE id = 1 LIMIT 1",).await?;
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct DogRecord {
        id: usize,
        name: Option<String>,
        section: Option<String>,
        breed: Option<String>,
        country: Option<String>,
        image: Option<String>,
        __createdtime__: usize,
        __updatedtime__: usize,
        age: Option<usize>,        
    }   
    
    let dog_record: Vec<DogRecord> = result.json().await?;
    println!("{:#?}", dog_record);

    Ok(())
}
