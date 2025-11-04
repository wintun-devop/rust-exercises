use uuid::Uuid;
use uuid_extra::new_v7;

pub fn uuid4() -> Uuid {
    Uuid::new_v4() 
}

pub fn uuid7() -> String {
    new_v7().to_string()
}