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
    const PI:f64 = 3.14159265359;
    println!("pi is {}.", PI);
    const SECONDS_IN_ONE_MINUTES:u32 = 60;
    println!("seconds in one minute is {}.", SECONDS_IN_ONE_MINUTES);
}
