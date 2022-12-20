// This is a simple program that prints "Hello, world!" to the console.

// function that takes a name (str) and says hello to it.
fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

pub fn run(name: &str) {
    println!("Hello from the `hello_world.rs` file.");
    say_hello(name);
}