use uuid::Uuid;
use uuid_extra::new_v7;
use std::{env};


pub fn uuid4() -> Uuid {
    Uuid::new_v4() 
}

pub fn uuid7() -> String {
    new_v7().to_string()
}

/* 
pub fn get_current_dir_string() -> Result<String, String> {
    match env::current_dir() {
        Ok(path) => match path.into_os_string().into_string() {
            Ok(s) => Ok(s),
            Err(_) => Err("Failed to convert path to String".to_string()),
        },
        Err(e) => Err(format!("Failed to get current directory: {}", e)),
    }
} 
*/


pub fn get_current_dir_string() -> Result<String, String> {
    env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))
        .and_then(|p| p.into_os_string().into_string().map_err(|_| "Failed to convert path to String".to_string()))
}

