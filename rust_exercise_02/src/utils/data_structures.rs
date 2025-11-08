use std::collections::HashMap;
use serde::Serialize;


pub fn static_string_array() -> [&'static str; 3] {
    ["Alice", "Carol", "Bob"]
}

/*
pub fn variable_string_array() -> Vec<String> {
    let mut my_array = Vec::new();
    my_array.push("Alice".to_string());
    my_array.push("Bob".to_string());
    my_array.push("Carol".to_string());
    my_array
}

*/

/*

pub fn boxed_names() -> Box<[String]> {
    let v = vec!["Alice".to_string(), "Bob".to_string()];
    v.into_boxed_slice()
}
*/

pub fn loop_array_function() {
    let fruits = ["apple", "banana", "orange"];
    // element only
    for fruit in fruits {
        println!("{}", fruit);
    }
    // index and element
    for (i, fruit) in fruits.iter().enumerate() {
        println!("{}:{}", i, fruit);
    }
    // alternative: index-based loop
    for i in 0..fruits.len() {
        println!("{}:{}", i, fruits[i]);
    }
}

pub fn tuple_test_function() {
    let person = ("Win Tun", 38, false);
    let (name, age, is_married) = person;
    println!("name:{}", name);
    println!("age:{}", age);
    println!("maried:{}", is_married);
}

#[derive(Serialize)]
#[serde(untagged)]
enum HashMashValue {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    // add variants as needed
}

pub fn test_hashmap_function() -> Result<(), Box<dyn std::error::Error>> {
    let mut capital_cities_only_string: HashMap<&str, &str>= HashMap::new();
    capital_cities_only_string.insert("England", "London");
    capital_cities_only_string.insert("Germany", "Berlin");
    capital_cities_only_string.insert("Norway", "Oslo");
    println!("{:?}", capital_cities_only_string);

    let mut mixed_hashmap:HashMap<&str, HashMashValue> = HashMap::new();
    mixed_hashmap.insert("name", HashMashValue::Str("Win Tun".to_string()));
    mixed_hashmap.insert("age", HashMashValue::Int(38));
    mixed_hashmap.insert("is_married", HashMashValue::Bool(false));
    mixed_hashmap.insert("some_thing", HashMashValue::Float(2.8));
    let s = serde_json::to_string_pretty(&mixed_hashmap)?;
    println!("{}", s);
    Ok(())
}
