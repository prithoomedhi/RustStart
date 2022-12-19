mod print;
mod vars;
mod check_if_prime;

fn main() {
    let num: i128 = 1024;
    print::run();
    vars::run();
    check_if_prime::run(num);
}
