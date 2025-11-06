mod services;

// need to import major module
mod utils;
use utils::uuids::{uuid4, uuid7,get_current_dir_string};
mod config;
use config::config;

fn main() {
    let a =52.6;
    println!("a is {}.type of a is {}.",a,services::get_type(&a));
    let u4 =uuid4();
    println!("{}",u4);
    let u7 =uuid7();
    println!("{}",u7);
    let api_secret = config().app_secret_key;
    println!("{}",api_secret);
    let jwt_secret = config().jwt_token_secret_key;
    println!("{}",jwt_secret);
    match get_current_dir_string() {
        Ok(dir) => println!("Current directory: {}", dir),
        Err(err) => eprintln!("Error: {}", err),
    }
}
