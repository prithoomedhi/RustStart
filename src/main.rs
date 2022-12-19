mod print;
mod vars;
mod check_if_prime;
mod prime_checker_v2;

fn main() {
    let num: i128 = 4096;
    let option:i128 = 2;
    print::run();
    vars::run();
    check_if_prime::run(num);
    prime_checker_v2::run(option, num);
}
