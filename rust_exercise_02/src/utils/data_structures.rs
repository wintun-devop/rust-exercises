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
