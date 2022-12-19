pub fn check_if_prime(num: i128) -> bool{
    let upper_limit: i128 = (num/2) + 1;
    let mut divisor: i128 = 2;
    let mut flag:bool = true;

    let mut factors: Vec<i128> = Vec::new();
    let known_prime_numbers: [i128; 7] = [2,3,5,7,11,13,17];

    if known_prime_numbers.contains(&num){
        return true;
    }

    while divisor < upper_limit{
        if num%divisor==0{
            factors.push(divisor);
        }
        divisor = divisor + 1;
    }

    if factors.len() > 0{
        flag = false;
    }

    return flag;
}

pub fn run(num: i128){
    println!("\nHello from the `check_if_prime.rs` file.");
    let result:bool = check_if_prime(num);
    if result == true{
        println!("{} is a prime number.", num)
    }
    else {
        println!("{} is not a prime number.", num)
    }
} 