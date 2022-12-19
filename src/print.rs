
pub fn run(){
    // Decalaring Fn variables
    let name = "John";
    let place = "Latveria";
    let activity = "code";
    let game = "Witcher 3: The Wild Hunt";
    let number: i128 = 512;

    // Run to console
    println!("\nHello from the `print.rs` file.");
    // String literals
    println!("Number:\t{}", 1);
    // More string literals
    println!("{} is from {}", name, place);

    // Positional arguments
    println!("{0} is from {1} amd {0} likes to {2}.", name, place, activity);

    // Named arguments
    println!("{name} like to play {game}.", name = name, game = game );

    // Placeholder traits
    println!("Binary: b{:b}\tHexadecimal: x0{:x}\tOctal: x8{:o}", number, number, number);

    // Placeholder for DEBUG trait
    println!("DEBUG: {:?}", (12, true, "hello"));

    // Basic math
    println!("{number} + {number} = {sum}", number=number, sum=number+number);
}