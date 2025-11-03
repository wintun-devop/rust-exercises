fn main() {
    // variable and its outputs
    let name = "John";
    let age = 30;
    println!("{} is {} years old.", name, age);

    // variable redeclaration ok
    let x = 20;
    println!("x is {}.", x);
    let x = 30;
    println!("x is {}.", x);
    let x = x + 1;
    println!("x is {}.", x);
}
