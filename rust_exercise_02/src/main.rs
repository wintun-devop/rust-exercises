mod services;


fn main() {
    let a =52.6;
    println!("a is {}.type of a is {}.",a,services::get_type(&a));
}
