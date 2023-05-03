use crate::constants::KNOWN_PRIMES;
use crate::utils::unique_elements_vector;

fn check_if_prime(num: i128) -> (bool, Vec<i128>){
    let known_prime_numbers: [i128; 7] = KNOWN_PRIMES; //List of known prime number; reduces processing time.

    let upper_limit: i128 = (num/2) + 1; //We do not need to check beyond the num/2 factor as it will be its highest possible factor.
    let mut divisor: i128 = 2; //Initialize the factor as 2.
    let mut flag:bool = true; //Default state of the flag: true-> isPrime; false-> notPrime.

    let mut factors: Vec<i128> = Vec::new(); //Initialize an empty list of factors.

    if known_prime_numbers.contains(&num){
        flag = true;
        factors.push(1);
        factors.push(num);
        return (flag,factors);
    }

    // Check all possible factors from 2 to num/2 and if a factor is found, add that value to the list of known factors.
    while divisor < upper_limit{
        if num%divisor==0{
            factors.push(divisor);
        }
        divisor = divisor + 1;
    }

    // If the list of known factors is not 0, then set flag as false.
    if factors.len() == 0{
        factors.push(1);
        factors.push(num);
        println!("Factors of {num}: {factors:?}", num=num, factors=factors);
    }
    else if factors.len() > 0{
        let mut new_factors: Vec<i128> = Vec::new();
        
        // Construct a new vector with 1 and num as the first and last elements respectively.
        new_factors.push(1);
        for item in factors{
            new_factors.push(item);
        }
        new_factors.push(num);

        // Update the list of factors to the new list of factors.
        factors = new_factors;

        println!("Factors of {num}: {factors:?}", num=num, factors=factors);
        flag = false;
    }

    // Return flag and list of factors.
    return (flag, factors);
}

fn check_one_number(num: i128){
    let result:bool;
    let factors:Vec<i128>;

    (result, factors) = check_if_prime(num);

    if result==false{
        println!("{num} is not a prime number; it's factors are: {factors:?}", num=num, factors=factors);
    }

    else {
        println!("{num} is a prime number; its only factors are {factors:?}.", num=num, factors=factors)
    }

}

fn check_till(num: i128)->Vec<i128>{
    let mut prime_numbers:Vec<i128> = Vec::new();
    for item in KNOWN_PRIMES{
        prime_numbers.push(item);
    }

    if num <=5 {
        println!("Please enter a number greater than 5.");
        return Vec::new();
    }

    println!("{:?} are commonly known prime numbers; skipping checking them individually...", prime_numbers);

    let start:i128 = 6;

    let mut result:bool;
    let mut _factors:Vec<i128> = Vec::new();

    for item in start..num+1{
        (result, _factors) = check_if_prime(item);

        if result == true{
            prime_numbers.push(item);
        }
    }
    prime_numbers = unique_elements_vector(prime_numbers);
    prime_numbers.sort();
    return prime_numbers;

}

pub fn run(option:i128, num:i128){
    // 1. Check if the number is a prime number and if not, display the set of factors.
    // 2. Find all prime numbers until the given number.

    println!("\nHello from the `prime_checker_v2.rs` file.");

    if option == 1{
        check_one_number(num);
    }
    else if option == 2{
        let primes: Vec<i128> = check_till(num);
        println!("The list of prime numbers til {} is {:?}.", num, primes);
    }
    else {
        println!("Invalid option.");
    }
}