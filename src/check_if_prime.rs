pub fn check_if_prime(num: i128) -> bool{
    let upper_limit: i128 = (num/2) + 1; //We do not need to check beyond the num/2 factor as it will be its highest possible factor.
    let mut divisor: i128 = 2; //Initialize the factor as 2.
    let mut flag:bool = true; //Default state of the flag: true-> isPrime; false-> notPrime.

    let mut factors: Vec<i128> = Vec::new(); //Initialize an empty list of factors.
    let known_prime_numbers: [i128; 7] = [2,3,5,7,11,13,17]; //List of known prime number; reduces processing time.

    // If the passed number is in the array of known prime numbers, immediately return true, no need to check further.
    if known_prime_numbers.contains(&num){
        return true;
    }

    // Check all possible factors from 2 to num/2 and if a factor is found, add that value to the list of known factors.
    while divisor < upper_limit{
        if num%divisor==0{
            factors.push(divisor);
        }
        divisor = divisor + 1;
    }

    // If the list of known factors is not 0, then set flag as false.
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