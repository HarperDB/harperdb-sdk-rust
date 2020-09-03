use harperdb_sdk_rust::{Harper, HarperConfig};


pub fn get_client() -> Harper {
    let config: HarperConfig = HarperConfig {
        url: "http://0.0.0.0:9925/",
        username: "HDB_ADMIN",
        password: "password",
        schema: "shop",
    };

    let harper_client = Harper::new(config);

    return harper_client;
}