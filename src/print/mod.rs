//! This is a print.rs file. It is used to print to the console.

use crate::constants;
use crate::utils;

pub fn run() {
    // Decalaring Fn variables
    let name = "John";
    let place = "Latveria";
    let activity = "code";
    let game = "Witcher 3: The Wild Hunt";
    let _min: i128 = constants::MIN_R;
    let _max: i128 = constants::MAX_R;
    let _length: usize = (_max - _min).abs() as usize;
    // let choices: Vec<i128> = utils::get_random_list(_min, _max, _length);
    // let number: i128 = utils::choose_random_element_from_vector(choices);
    let number: i128 = utils::random_integer(_min, _max);

    // Run to console
    println!("\nHello from the `print.rs` file.");
    // String literals
    println!("Number:\t{}", 1);
    // More string literals
    println!("{} is from {}", name, place);

    // Positional arguments
    println!(
        "{0} is from {1} amd {0} likes to {2}.",
        name, place, activity
    );

    // Named arguments
    println!("{name} like to play {game}.", name = name, game = game);

    // Placeholder traits
    println!(
        "Number: {:?}\tBinary: b{:b}\tHexadecimal: x0{:x}\tOctal: x8{:o}",
        number, number, number, number
    );

    // Placeholder for DEBUG trait
    println!("DEBUG: {:?}", (12, true, "hello"));

    // Basic math
    println!(
        "{number} + {number} = {sum}",
        number = number,
        sum = number + number
    );
}
