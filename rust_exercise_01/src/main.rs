use serde_json::{Value, json};
use std::any::type_name;

fn main() {
    // variable and its outputs
    let name = "John";
    let age = 30;
    println!("{} is {} years old.", name, age);

    // variable redeclaration ok
    let x = 20;
    println!("x is {}.", x);
    let x = 30;
    {
        /* own scope variable */
        let x = 111;
        println!("own scop x is {}.", x);
    }
    println!("x is {}.", x);
    let x = x + 1;
    println!("x is {}.", x);
    /* constants */
    const PI: f64 = 3.14159265359;
    println!("pi is {}.", PI);
    const SECONDS_IN_ONE_MINUTES: u32 = 60;
    println!("seconds in one minute is {}.", SECONDS_IN_ONE_MINUTES);

    let a = var_list();
    println!("{}", a);
    typ_function();
}

// simple rust function
fn var_list() -> Value {
    let obj: Value = json!({
      "name": "Win",
      "score": 42,
      "tags": ["rust", "backend"]
    });
    return obj;
}


fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}


// type function
fn typ_function() {
    //0 ot 255
    let a: u8 = 12;
    println!("a is {}.type is {}.",a,type_of(&a));
    // -128 to 127
    let b:i8 = -100;
    println!("b is {}.type is {}.",b,type_of(&b));
    let c = 2.1;
    println!("c is {}.type is {}.",c,type_of(&c));

}

