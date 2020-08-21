use reqwest::Error;

use serde_json::Value;
use std::collections::HashMap;

#[macro_use]
extern crate serde_json;
// pub mod harper_options {

use serde::{Deserialize, Serialize};

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
    pub records: Value, //----------------------------
    // pub records: Vec<serde_json::Value>, //----------------------------
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RowDeleteOptions {
    pub table: &'static str,
    pub schema: &'static str,
    // pub hash_values: Vec<serde_json::Value>,
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
pub struct S3SearchOperation {
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
    pub search_operation: S3SearchOperation,
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
    // pub attributes: Option<Vec<&'static str>>,
    // pub attributes: Option<Vec<String>>,
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
// }

pub struct Harper {
    config: HarperConfig,
    client: reqwest::Client,
}
impl Harper {
    pub fn new(harper_config: HarperConfig) -> Self {
        Harper {
            config: harper_config,
            client: reqwest::Client::new(),
        }
    }

    /// Create Schema
    /// 
    /// # Arguments
    /// 
    /// * `schema`  (required) - name of the schema you are creating
    /// 
    /// # Examples
    /// 
    /// ```
    /// let result = harper_client.cluster_status().await.unwrap();
    /// let data:Struct = result.json().await?;
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

    pub async fn system_information(&self, options: SystemInformationOptions) -> Result<reqwest::Response, Error> {
        // let mut map = HashMap::new();
        // map.insert("operation", "system_information");
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

    pub async fn export_to_local(&self, options: S3DetailsOptions) -> Result<reqwest::Response, Error> {
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

    pub async fn set_license(&self, options: LicenseOptions) -> Result<reqwest::Response, Error> {
        let mut map = HashMap::new();
        map.insert("operation", "get_fingerprint");
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

    /// System Information 
    /// # Arguments
    /// 
    /// * `attributes` (optional) - string array of top level attributes desired in the response, if no value is supplied all attributes will be returned.
    ///  Available attributes are: ['system', 'time', 'cpu', 'memory', 'disk', 'network', 'harperdb_processes']
    /// 
    /// A deeper dive into the return object can be found here: https://systeminformation.io/general.html
    /// 
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