/*
    1. Variables hold data or references to data.
    2. Variable are IMmutable BY DEFAULT.
    3. RUST is a block-scoped language.
*/

pub fn run() {
    // Run to console
    println!("\nHello from the `vars.rs` file.");

    let name = "John";
    let _age: i128 = 25;
    println!("My name is {} and I am {} years old.", name, _age);
}
