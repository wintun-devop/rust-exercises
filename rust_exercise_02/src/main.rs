mod services;
// need to import major module
mod utils;
use utils::uuids::{uuid4, uuid7};

fn main() {
    let a =52.6;
    println!("a is {}.type of a is {}.",a,services::get_type(&a));
    let u4 =uuid4();
    println!("{}",u4);
    let u7 =uuid7();
    println!("{}",u7);
}
