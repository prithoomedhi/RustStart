#![allow(warnings)]

mod print;
mod vars;
mod prime_checker_v2;
mod hello_world;
mod utils;
mod constants;

fn main() {
    let num: i128 = utils::random_integer(constants::MIN_R, constants::MAX_R);
    let option:i128 = utils::random_integer(1, 3);
    // let num: i128 = 60;
    // let option:i128 = 1;
    // let name = "John";
    // hello_world::run(name);
    // print::run();
    // vars::run();
    prime_checker_v2::run(option, num);
}
