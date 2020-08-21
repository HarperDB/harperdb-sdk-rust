//! # A Rust Client for HarperDB
//! ## Introduction
//!
//! HarperDB is a SQL/NoSQL data management platform. It is fully indexed, doesn't duplicate data, and runs on any device- from the edge to the cloud.
//!
//! It is built natively as a set of micro-services, making development and integration easy and seamless. HarperDB utilizes a single-endpoint for all operations. HarperDB’s RESTful nature makes it stateless, stable, and scalable.
//!
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! use harperdb_sdk_rust::{ HarperConfig, Harper };
//! use harperdb_sdk_rust as harper;
//! use serde::{Deserialize, Serialize};
//! use serde_json::{Value};
//! use std::{error::Error};
//! 
//! #[macro_use]
//! extern crate serde_json;
//! 
//! #[derive(Debug, Serialize, Deserialize)]
//! struct DogRecord {
//!     id: usize,
//!     name: String,
//!     age: Option<usize>, 
//!     breed: Option<String>,
//!     image: Option<String>,
//!     __createdtime__: usize,
//!     __updatedtime__: usize,
//! }
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let config: HarperConfig = HarperConfig {
//!         url: "http://0.0.0.0:9925/",
//!         username: "HDB_ADMIN",
//!         password: "password",
//!         schema: "dev",
//!     };
//!
//!     let harper_client = new(config);
//!     
//!     // Insert Record
//!     let insert_option: harper::QueryOptions = harper::QueryOptions {
//!         table: "dog",
//!         schema: "dev",
//!         records:json!([{
//!             "id": 1,
//!             "name": "Incredible Metal Chair",
//!             "breed": "Mutt",
//!             "age": 4,
//!             "image": "http://lorempixel.com/640/480/nature"
//!         }]),
//!     };
//!     let result = harper_client.insert(insert_option).await?;
//!     
//!     // Get Query Response
//!     println!("{:#?}", result.status());
//!     
//!     // Query Database
//!     let result = harper_client.query("SELECT * FROM dev.dog limit 2",).await?;
//!    
//!     let dog_record: Vec<DogRecord> = result.json().await?;
//!     println!("{:#?}", dog_record);
//! 
//!     // Get result as text
//!     // let data = result.text().await?;
//!     // println!("{:#?}", data);
//! 
//! }
//! ```

use reqwest::Error;
use serde_json::Value;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HarperConfig {
    pub url: &'static str,
    pub username: &'static str,
    pub password: &'static str,
    pub schema: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseData {
    pub status: &'static str,
    pub status_code: &'static str,
    pub data: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaOption {
    pub schema: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableOptions {
    pub table: &'static str,
    pub schema: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTableOptions {
    pub table: &'static str,
    pub schema: &'static str,
    pub hash_attribute: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttributeDropOptions {
    pub table: &'static str,
    pub schema: &'static str,
    pub attribute: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryOptions {
    pub table: &'static str,
    pub schema: &'static str,
    pub records: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RowDeleteOptions {
    pub table: &'static str,
    pub schema: &'static str,
    pub hash_values: Vec<&'static str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HashSearchOptions {
    pub table: &'static str,
    pub schema: &'static str,
    pub hash_values: Vec<&'static str>,
    pub get_attributes: Vec<&'static str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValueSearchOptions {
    pub table: &'static str,
    pub schema: &'static str,
    pub search_attribute: &'static str,
    pub search_value: &'static str,
    pub get_attributes: Vec<&'static str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataLoadOptions {
    pub table: &'static str,
    pub schema: &'static str,
    pub action: &'static str,
    pub data: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlLoadOptions {
    pub table: &'static str,
    pub schema: &'static str,
    pub action: &'static str,
    pub csv_url: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileLoadOptions {
    pub table: &'static str,
    pub schema: &'static str,
    pub action: &'static str,
    pub file_path: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAddOptions {
    pub role: &'static str,
    pub username: &'static str,
    pub password: &'static str,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAlterOptions {
    pub role: Option<&'static str>,
    pub username: &'static str,
    pub password: Option<&'static str>,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDropOptions {
    pub username: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddRoleOptions {
    pub role: &'static str,
    pub permission: serde_json::Value,
    pub super_user: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlterRoleOptions {
    pub role: &'static str,
    pub id: &'static str,
    pub permission: serde_json::Value,
    pub super_user: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DropRoleOptions {
    pub id: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteFilesBeforeOptions {
    pub schema: &'static str,
    pub table: &'static str,
    pub date: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchOperation {
    pub operation: &'static str,
    pub sql: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S3Auth {
    pub aws_access_key_id: &'static str,
    pub aws_secret_access_key: &'static str,
    pub bucket: &'static str,
    pub key: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S3DetailsOptions {
    pub format: &'static str,
    pub s3: S3Auth,
    pub search_operation: SearchOperation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportLocalOptions {
    pub format: &'static str,
    pub path: &'static str,
    pub search_operation: SearchOperation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AscDesc {
    Desc,
    Asc,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogsOptions {
    pub limit: Option<usize>,
    pub start: Option<usize>,
    pub from: Option<&'static str>,
    pub until: Option<&'static str>,
    pub order: Option<&'static str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetJobOptions {
    pub id: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobsByDateOptions {
    pub from_date: &'static str,
    pub to_date: &'static str,
}

#[derive(Debug, Serialize, Clone)]
pub struct SystemInformationOptions {
    pub attributes: Option<Vec<&'static str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LicenseOptions {
    key: &'static str,
    company: &'static str,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddNodeOptions {
    pub name: &'static str,
    pub port: &'static str,
    pub host: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateNodeOptions {
    pub name: &'static str,
    pub port: &'static str,
    pub host: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoveNodeOptions {
    pub name: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attribute {
    pub attribute: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DescribeTable {
    pub __createdtime__: usize,
    pub __updatedtime__: usize,
    pub hash_attribute: &'static str,
    pub id: &'static str,
    pub name: &'static str,
    pub residence: Option<&'static str>,
    pub schema: &'static str,
    pub record_count: usize,
    pub attributes: Vec<Attribute>,
}

pub struct Harper {
    config: HarperConfig,
    client: reqwest::Client,
}
impl Harper {
    /// Create a HaperDB client
    ///
    /// # Arguments
    ///
    /// * `harper_config`  (required) - HarperConfig
    ///
    /// # Examples
    ///
    /// ```
    /// let config: HarperConfig = HarperConfig {
    ///     url: "http://0.0.0.0:9925/",
    ///     username: "HDB_ADMIN",
    ///     password: "password",
    ///     schema: "shop",
    /// };
    ///
    /// let harper_client = new(config);
    /// ```
    pub fn new(harper_config: HarperConfig) -> Self {
        Harper {
            config: harper_config,
            client: reqwest::Client::new(),
        }
    }

    /// Create Schema:
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - SchemaOption
    /// 
    /// # Examples
    /// 
    /// ```
    /// let schema_option: SchemaOption = SchemaOption {
    ///     schema: "newschema",
    /// };
    /// let result = harper_client.create_schema(schema_option).await?;
    /// ```
    /// 
    pub async fn create_schema(&self, options: SchemaOption) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "create_schema");
        map.insert("schema", &options.schema);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Drop Schema:
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - SchemaOption
    /// 
    /// # Examples
    /// 
    /// ```
    /// let schema_option: SchemaOption = SchemaOption {
    ///     schema: "newschema",
    /// };
    /// let result = harper_client.drop_schema(schema_option).await?;
    /// ```
    /// 
    pub async fn drop_schema(&self, options: SchemaOption) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "drop_schema");
        map.insert("schema", &options.schema);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Describe Schema:
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - SchemaOption
    /// 
    /// # Examples
    /// 
    /// ```
    /// let schema_option: SchemaOption = SchemaOption {
    ///     schema: "newschema",
    /// };
    /// let result = harper_client.describe_schema(schema_option).await?;
    /// ```
    /// 
    pub async fn describe_schema(&self, options: SchemaOption) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "describe_schema");
        map.insert("schema", &options.schema);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Describe All
    /// 
    /// # Examples
    /// 
    /// ```
    /// let result = harper_client.describe_all().await?;
    /// ```
    /// 
    pub async fn describe_all(&self) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "describe_all");

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Create Table
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - CreateTableOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let create_table_option: CreateTableOptions = CreateTableOptions {
    ///     hash_attribute: "id",
    ///     table: "test_table",
    ///     schema: "describe_schema_test",
    /// };
    /// let result = harper_client.create_table(create_table_option).await?;
    /// ```
    /// 
    pub async fn create_table(
        &self,
        options: CreateTableOptions,
    ) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "create_table");
        map.insert("hash_attribute", &options.hash_attribute);
        map.insert("table", &options.table);
        map.insert("schema", &options.schema);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Describe Table
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - TableOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let table_option: TableOptions = TableOptions {
    ///     table: "tablename",
    ///     schema: "newschema",
    /// };
    /// let result = harper_client.Describe Table(table_option).await?;
    /// ```
    /// 
    pub async fn describe_table(&self, options: TableOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "describe_table");
        map.insert("table", &options.table);
        map.insert("schema", &options.schema);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;
        // let gist: DescribeTable = res.json().await?;
        // println!("{:#?}", gist);
        // let result = res.json().await?;
        // let v: Value = serde_json::from_str(result)?;

        Ok(res)
    }

    /// Drop Table Attribute
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - TableOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let schema_option: SchemaOption = SchemaOption {
    ///     schema: "newschema",
    /// };
    /// let result = harper_client.create_schema(schema_option).await?;
    /// ```
    /// 
    pub async fn drop_table(&self, options: TableOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "drop_table");
        map.insert("table", &options.table);
        map.insert("schema", &options.schema);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Drop Table Attribute
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - AttributeDropOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let attribute_drop_option: AttributeDropOptions = AttributeDropOptions {
    ///     table: "drop_attribute_table_test",
    ///     schema: "testing",
    ///     attribute: "breed",
    /// };
    /// let result = harper_client.drop_attribute(attribute_drop_option).await?;
    /// ```
    /// 
    pub async fn drop_attribute(&self, options: AttributeDropOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "drop_attribute");
        map.insert("table", &options.table);
        map.insert("schema", &options.schema);
        map.insert("attribute", &options.attribute);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Create Schema:
    /// 
    /// # Arguments
    /// 
    /// * `sql`  (required) - &str
    /// 
    /// # Examples
    /// 
    /// ```
    /// let result = harper_client.query("SELECT * FROM testing.crud_table_test limit 2",).await?;
    /// ```
    /// 
    pub async fn query(&self, sql_query: &'static str, ) -> Result<reqwest::Response, Error> {

        let map = json!({
            "operation": "sql",
            "sql": &sql_query,
        });

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Insert Records
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - QueryOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let insert_option: QueryOptions = QueryOptions {
    ///     table: "crud_table_test",
    ///     schema: "testing",
    ///     records:json!([{
    ///         "id": "record1234",
    ///         "name": "Incredible Metal Chair",
    ///         "modelNumber": 356625,
    ///         "unitPrice": "$1700.00",
    ///         "color": "#961",
    ///         "material": "Rubber",
    ///         "isDiscontinued": true,
    ///         "image": "http://lorempixel.com/640/480/nature"
    ///     }]),
    /// };
    /// let result = harper_client.insert(insert_option).await?;
    /// ```
    /// 
    pub async fn insert(&self, options: QueryOptions) -> Result<reqwest::Response, Error> {

        let map = json!({
            "operation": "insert",
            "table": &options.table,
            "schema": &options.schema,
            "records": &options.records
        });

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Update Records
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - QueryOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let update_option: QueryOptions = QueryOptions {
    ///     table: "crud_table_test",
    ///     schema: "testing",
    ///     records:json!([{            
    ///         "id": "record1234",           
    ///         "color": "red",            
    ///     }]),
    /// };
    /// let result = harper_client.update(update_option).await?;
    /// ```
    /// 
    pub async fn update(&self, options: QueryOptions) -> Result<reqwest::Response, Error> {

        let map = json!({
            "operation": "update",
            "table": &options.table,
            "schema": &options.schema,
            "records": &options.records
        });

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Delete Records:
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - RowDeleteOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let delete_option: RowDeleteOptions = RowDeleteOptions {
    ///     table: "crud_table_test",
    ///     schema: "testing",
    ///     hash_values:vec!["record1234"] ,//json!([]),
    /// };
    /// let result = harper_client.delete(delete_option).await?;
    /// ```
    /// 
    pub async fn delete(&self, options: RowDeleteOptions) -> Result<reqwest::Response, Error> {
        let map = json!({
            "operation": "delete",
            "table": &options.table,
            "schema": &options.schema,
            "hash_values": &options.hash_values
        });

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Search By Hash:
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - HashSearchOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let search_option: HashSearchOptions = HashSearchOptions {
    ///     table: "crud_table_test",
    ///     schema: "testing",
    ///     hash_values:vec![
    ///         "updaterecord1234",
    ///     ],
    ///     get_attributes:vec!["name"],
    /// };
    ///
    /// let result = harper_client.search_by_hash(search_option).await?;
    /// ```
    /// 
    pub async fn search_by_hash(&self, options: HashSearchOptions) -> Result<reqwest::Response, Error> {

        let map = json!({
            "operation": "search_by_hash",
            "table": &options.table,
            "schema": &options.schema,
            "hash_values": &options.hash_values,
            "get_attributes": &options.get_attributes
        });

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Search By Value:
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - ValueSearchOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let search_option: ValueSearchOptions = ValueSearchOptions {
    ///     table: "crud_table_test",
    ///     schema: "testing",
    ///     search_attribute: "name",
    ///     search_value:"Tom*",
    ///     get_attributes:vec!["name"],
    /// };
    ///
    /// let result = harper_client.search_by_value(search_option).await?;
    /// ```
    /// 
    pub async fn search_by_value(&self, options: ValueSearchOptions) -> Result<reqwest::Response, Error> {

        let map = json!({
            "operation": "search_by_value",
            "table": &options.table,
            "schema": &options.schema,
            "search_attribute": &options.search_attribute,
            "search_value": &options.search_value,
            "get_attributes": &options.get_attributes
        });

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// CSV Data Load
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - DataLoadOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let csv_data_load_option: DataLoadOptions = DataLoadOptions {
    ///     table: "crud_table_test",
    ///     schema: "testing",
    ///     action: "insert",
    ///     data: "id,name,section,country,image\n1,ENGLISH POINTER,British and Irish Pointers and Setters,GREAT BRITAIN,http://www.fci.be/Nomenclature/Illustrations/001g07.jpg\n2,ENGLISH SETTER,British and Irish Pointers and Setters,GREAT BRITAIN,http://www.fci.be/Nomenclature/Illustrations/002g07.jpg\n3,KERRY BLUE TERRIER,Large and medium sized Terriers,IRELAND,\n",        
    /// };
    ///
    /// let result = harper_client.csv_data_load(csv_data_load_option).await?
    /// ```
    /// 
    pub async fn csv_data_load(&self, options: DataLoadOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "csv_data_load");
        map.insert("table", &options.table);
        map.insert("schema", &options.schema);
        map.insert("action", &options.action);
        map.insert("data", &options.data);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// CSV URL Load:
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) -
    /// 
    /// # Examples
    /// 
    /// ```
    /// let csv_data_load_option: FileLoadOptions = FileLoadOptions {
    ///     table: "crud_table_test",
    ///     schema: "testing",
    ///     action: "insert",
    ///     file_path: "~/Codes/harperdb-sdk-rust/breeds.csv"
    /// };
    ///
    /// let result = harper_client.csv_file_load(csv_data_load_option).await?;
    /// ```
    /// 
    pub async fn csv_url_load(&self, options: UrlLoadOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "csv_url_load");
        map.insert("table", &options.table);
        map.insert("schema", &options.schema);
        map.insert("action", &options.action);
        map.insert("csv_url", &options.csv_url);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// CSV URL Load:
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - FileLoadOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let csv_data_load_option: FileLoadOptions = FileLoadOptions {
    ///     table: "crud_table_test",
    ///     schema: "testing",
    ///     action: "insert",
    ///    file_path: "~/Codes/harperdb-sdk-rust/breeds.csv"
    /// };
    ///
    /// let result = harper_client.csv_file_load(csv_data_load_option).await?;
    /// ```
    /// 
    pub async fn csv_file_load(&self, options: FileLoadOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "csv_file_load");
        map.insert("table", &options.table);
        map.insert("schema", &options.schema);
        map.insert("action", &options.action);
        map.insert("file_path", &options.file_path);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }


    /// List Users:
    /// 
    /// # Examples
    /// 
    /// ```
    /// let result = harper_client.list_users().await?;
    /// ```
    /// 
    pub async fn list_users(&self) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "list_users");

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// User Info
    /// 
    /// # Examples
    /// 
    /// ```
    /// let result = harper_client.user_info().await?;
    /// ```
    ///  
    pub async fn user_info(&self) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "user_info");

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Add User
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - UserAddOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let user_option: UserAddOptions = UserAddOptions {
    ///     role: "c0a90733-1fc3-48df-a16b-d7c3011b63b2",
    ///     username: "john",
    ///     password: "secret",
    ///     active: true
    /// };
    /// let result = harper_client.add_user(user_option).await?;
    /// ```
    /// 
    pub async fn add_user(&self, options: UserAddOptions) -> Result<reqwest::Response, Error> {

        let map = json!({
            "operation": "add_user",
            "role": &options.role,
            "username": &options.username,
            "password": &options.password,
            "active": &options.active
        });

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Alter User
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - UserAlterOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let user_option: UserAlterOptions = UserAlterOptions {
    ///     role: Some("c0a90733-1fc3-48df-a16b-d7c3011b63b2"),
    ///     username: "john",
    ///     password: Some("secret2"),
    ///     active: true
    /// };
    /// let result = harper_client.alter_user(user_option).await?;
    /// ```
    /// 
    pub async fn alter_user(&self, options: UserAlterOptions) -> Result<reqwest::Response, Error> {
        let map = json!({
            "operation": "alter_user",
            "role": &options.role,
            "username": &options.username,
            "password": &options.password,
            // "password": &options.password,
            "active": &options.active
        });

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Drop User 
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - UserDropOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let user_option: UserDropOptions = UserDropOptions {
    ///     username: "john",
    /// };
    /// let result = harper_client.drop_user(user_option).await?;
    /// ```
    /// 
    pub async fn drop_user(&self, options: UserDropOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "drop_user");
        map.insert("username", &options.username);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// List Roles
    /// 
    /// # Examples
    /// 
    /// ```
    /// let result = harper_client.list_roles().await?;;
    /// ```
    /// 
    pub async fn list_roles(&self) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "list_roles");

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Add Role
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - AddRoleOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let role_option: AddRoleOptions = AddRoleOptions {
    ///     role: "develope3r",
    ///     super_user: false,
    ///     permission:json!({
    ///         "testing":{
    ///             "tables": {
    ///               "crud_table_test": {
    ///                   "read":true,
    ///                   "insert":false,
    ///                   "update":false,
    ///                   "delete":false,
    ///                   "attribute_restrictions":[
    ///                      {
    ///                         "attribute_name": "color",
    ///                         "read":false,
    ///                         "insert":false,
    ///                         "update":false,
    ///                         "delete":false
    ///                      }
    ///                   ]
    ///                }
    ///             }
    ///          }
    ///     }),
    // };
    ///
    /// let result = harper_client.add_role(role_option).await?;
    /// ```
    /// 
    pub async fn add_role(&self, options: AddRoleOptions) -> Result<reqwest::Response, Error> {

        let mut map = json!({
            "operation": "add_role",
            "role": &options.role,
            "permission": &options.permission            
        });

        map["permission"]["super_user"]=Value::Bool(options.super_user);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Alter Role
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - AlterRoleOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let role_option: AlterRoleOptions = AlterRoleOptions {
    ///     id: "3c5cc923-5351-4f81-91e3-01a03448e18f",
    ///     role: "cluster_user",
    ///     super_user: false,
    ///     permission:json!({
    ///         "shop":{
    ///             "tables": {
    ///               "product": {
    ///                   "read":false,
    ///                   "insert":true,
    ///                   "update":true,
    ///                   "delete":true,
    ///                   "attribute_restrictions":[
    ///                      {
    ///                         "attribute_name": "color",
    ///                         "read":false,
    ///                         "insert":true,
    ///                         "update":true,
    ///                         "delete":false
    ///                      }
    ///                   ]
    ///                }
    ///             }
    ///          }
    ///     }),
    /// };
    /// let result = harper_client.alter_role(role_option).await?;
    /// ```
    /// 
    pub async fn alter_role(&self, options: AlterRoleOptions) -> Result<reqwest::Response, Error> {
        let mut map = json!({
            "operation": "alter_role",
            "id":&options.id,
            "role": &options.role,
            "permission": &options.permission            
        });

        map["permission"]["super_user"]=Value::Bool(options.super_user);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Drop Role
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - DropRoleOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let role_option: DropRoleOptions = DropRoleOptions {
    ///     id: "33d285dc-1ddb-4700-b5bd-300a67faa247",
    /// };
    /// let result = harper_client.drop_role(role_option).await?;
    /// ```
    /// 
    pub async fn drop_role(&self, options: DropRoleOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "drop_role");
        map.insert("id", &options.id);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// System Information
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - SystemInformationOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let system_information_option: SystemInformationOptions = SystemInformationOptions {
    ///     attributes : Some(vec!["cpu"])
    ///     // attributes :None
    /// };
    /// let result = harper_client.system_information(system_information_option).await?;
    /// ```
    /// 
    pub async fn system_information(&self, options: SystemInformationOptions) -> Result<reqwest::Response, Error> {
        let map = json!({
            "operation": "system_information",
            "attributes": &options.attributes            
        });

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Delete Files Before
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - DeleteFilesBeforeOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let option: DeleteFilesBeforeOptions = DeleteFilesBeforeOptions {
    ///     date: "2018-07-10",
    ///     schema: "dev",
    ///     table: "breed",
    /// };
    /// let result = harper_client.delete_files_before(option).await?;
    /// ```
    ///
    pub async fn delete_files_before(&self, options: DeleteFilesBeforeOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "delete_files_before");
        map.insert("table", &options.table);
        map.insert("schema", &options.schema);
        map.insert("date", &options.date);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Export To S3
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - S3DetailsOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let option: S3DetailsOptions = S3DetailsOptions {
    ///     format: "json",
    ///     s3:S3Auth{
	///	        aws_access_key_id:"YOUR_KEY",
	///	        aws_secret_access_key:"YOUR_SECRET_KEY",
	///	        bucket:"BUCKET",
	///	        key:"FILENAME"
	///      }, 
    ///     search_operation: SearchOperation{
    ///         "operation": "sql",
    ///         "sql": "SELECT * FROM dev.dog"
    ///       }
    /// };
    /// let result = harper_client.export_to_local(option).await?;
    /// 
    pub async fn export_to_s3(&self, options: S3DetailsOptions) -> Result<reqwest::Response, Error> {
        let map = json!({
            "operation": "export_to_s3",
            "format": &options.format,
            "s3":&options.s3,
            "search_operation":&options.search_operation,                        
        });

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Export Local
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) -ExportLocalOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let option: ExportLocalOptions = ExportLocalOptions {
    ///     format: "json",
    ///     s3:"/data/", 
    ///     search_operation: SearchOperation{
    ///         "operation": "sql",
    ///         "sql": "SELECT * FROM dev.dog"
    ///       }
    /// };
    /// let result = harper_client.export_to_local(option).await?;
    /// ```
    ///
    pub async fn export_to_local(&self, options: ExportLocalOptions) -> Result<reqwest::Response, Error> {
        let map = json!({
            "operation": "export_local",
            "format": &options.format,
            "path":&options.path,
            "search_operation":&options.search_operation,                        
        });

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Read Log
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - LogsOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let read_logs_option: LogsOptions = LogsOptions {
    ///     limit : Some(2),
    ///     start : Some(0),
    ///     from : None,
    ///     until : None,
    ///     order : None,    
    /// };
    /// let result = harper_client.read_logs(read_logs_option).await?;
    /// ```
    /// 
    pub async fn read_logs(&self, options: LogsOptions) -> Result<reqwest::Response, Error> {
        let map = json!({
            "operation": "read_log",
            "limit": &options.limit,
            "start":&options.start,
            "from":&options.from,
            "until":&options.until,
            "order":&options.order,
        });
        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Get Job
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - GetJobOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let get_job_option: GetJobOptions = GetJobOptions {
    ///     id : "d8b70ed4-a62a-45ef-bf86-15508c4ba10a",
    /// };
    ///
    /// let result = harper_client.get_job(get_job_option).await?;
    /// ```
    /// 
    pub async fn get_job(&self, options: GetJobOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "get_job");
        map.insert("id", &options.id);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Search Jobs By Start Date
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - JobsByDateOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let search_jobs_by_start_date_option: JobsByDateOptions = JobsByDateOptions {
    ///     from_date : "2019-01-01",
    ///     to_date : "2020-12-30",
    /// };
    ///
    /// let result = harper_client.search_jobs_by_start_date(search_jobs_by_start_date_option).await?;
    /// ```
    /// 
    pub async fn search_jobs_by_start_date(&self, options: JobsByDateOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "search_jobs_by_start_date");
        map.insert("from_date", &options.from_date);
        map.insert("to_date", &options.to_date);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }










    /// Registration Info 
    /// 
    /// # Examples
    /// 
    /// ```    
    /// let result = harper_client.registration_info().await?;
    /// ```
    /// 
    pub async fn registration_info(&self) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "registration_info");

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Get Fingerprint:
    /// 
    /// # Examples
    /// 
    /// ```
    /// let result = harper_client.get_fingerprint().await?;
    /// ```
    /// 
    pub async fn get_fingerprint(&self) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "get_fingerprint");

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Set License
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - LicenseOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let option: LicenseOptions = LicenseOptions {
    ///     key: "<your-license-key>",
    ///     company: "<your-company>",
    /// };
    /// let result = harper_client.set_license(option).await?;
    /// ```
    /// 
    pub async fn set_license(&self, options: LicenseOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "set_license");
        map.insert("key", &options.key);
        map.insert("company", &options.company);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }


    /// Add Node
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - AddNodeOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let option: AddNodeOptions = AddNodeOptions {
    ///     name: "node2",
    ///     port: 9925,
    ///     host: "192.168.100.100",
    /// };
    /// let result = harper_client.add_node(option).await?;
    /// ```
    /// 
    pub async fn add_node(&self, options: AddNodeOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "add_node");
        map.insert("name", &options.name);
        map.insert("port", &options.port);
        map.insert("host", &options.host);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }


    /// Update Node
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - UpdateNodeOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let option: UpdateNodeOptions = UpdateNodeOptions {
    ///     name: "node2",
    ///     port: 9925,
    ///     host: "192.168.100.100",
    /// };
    /// let result = harper_client.update_node(option).await?;
    /// ```
    /// 
    pub async fn update_node(&self, options: UpdateNodeOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "add_node");
        map.insert("name", &options.name);
        map.insert("port", &options.port);
        map.insert("host", &options.host);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }


    /// Remove Node
    /// 
    /// # Arguments
    /// 
    /// * `options`  (required) - RemoveNodeOptions
    /// 
    /// # Examples
    /// 
    /// ```
    /// let option: RemoveNodeOptions = RemoveNodeOptions {
    ///     name: "nodename",
    /// };
    /// let result = remove_node.remove_node(option).await?;
    /// ```
    /// 
    pub async fn remove_node(&self, options: RemoveNodeOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "remove_node");
        map.insert("name", &options.name);

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }

    /// Cluster Status
    /// 
    /// # Examples
    /// ```
    /// let result = harper_client.cluster_status().await.unwrap();
    /// let data:Struct = result.json().await?;
    /// ```
    ///     
    pub async fn cluster_status(&self) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "cluster_status");

        let res = self
            .client
            .post(self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&map)
            .send()
            .await?;

        Ok(res)
    }
}