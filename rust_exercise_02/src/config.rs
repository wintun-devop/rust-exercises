use dotenvy::dotenv;
use std::{env,path::Path};

pub struct Config {
    pub app_secret_key: String,
    pub jwt_token_secret_key:String,
    // pub db_write_host:String,
    // pub db_read_host:String,
    // pub db_user:String,
    // pub db_password:String,
    // pub db_name:String
}

pub fn config() -> Config {
    dotenv().ok();
    Config {
        app_secret_key: env::var("APP_SECRET_KEY").expect("Env Read Error."),
        jwt_token_secret_key:env::var("JWT_TOKEN_SECRET_KEY").expect("Env Read Error."),
        // db_write_host:env::var("DB_WRTIE_HOST").expect("Env Read Error."),
        // db_read_host:env::var("DB_READ_HOST").expect("Env Read Error."),
        // db_user:env::var("DB_USER").expect("Env Read Error."),
        // db_password:env::var("DB_PASSWORD").expect("Env Read Error."),
        // db_name:env::var("DB_NAME").expect("Env Read Error.")
    }
}


pub fn project_dir() -> String {
    // file!() is the path to the current source file as known at compile time.
    // env!("CARGO_MANIFEST_DIR") is the crate root absolute path at compile time.
    let absolute = format!("{}\\{}", env!("CARGO_MANIFEST_DIR"), file!());
    let p = Path::new(&absolute);
    p.parent()
     .map(|d| d.to_string_lossy().into_owned())
     .unwrap_or_else(|| ".".to_string())
}