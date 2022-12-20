fn check_if_prime(num: i128) -> (bool, Vec<i128>){
    let known_prime_numbers: [i128; 7] = [2,3,5,7,11,13,17]; //List of known prime number; reduces processing time.

    let upper_limit: i128 = (num/2) + 1; //We do not need to check beyond the num/2 factor as it will be its highest possible factor.
    let mut divisor: i128 = 2; //Initialize the factor as 2.
    let mut flag:bool = true; //Default state of the flag: true-> isPrime; false-> notPrime.

    let mut factors: Vec<i128> = Vec::new(); //Initialize an empty list of factors.

    if known_prime_numbers.contains(&num){
        flag = true;
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
    if factors.len() > 0{
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
        println!("{} is not a prime number; it's factors are: {:?}", num, factors);
    }

    else {
        println!("{num} is a prime number.")
    }

}

fn check_till(num: i128){
    let mut prime_numbers:Vec<i128> = Vec::new();
    prime_numbers.push(2);
    prime_numbers.push(3);
    prime_numbers.push(5);

    if num <=5 {
        println!("Please enter a number greater than 5.");
        return
    }

    let start:i128 = 6;

    let mut result:bool;
    let mut _factors:Vec<i128> = Vec::new();

    for i in start..num+1{
        (result, _factors) = check_if_prime(i);

        if result == true{
            prime_numbers.push(i);
        }
    }

    println!("The list of prime numbers til {} is {:?}.", num, prime_numbers);
    return

}

pub fn run(option:i128, num:i128){
    // 1. Check if the number is a prime number and if not, display the set of factors.
    // 2. Find all prime numbers until the given number.

    println!("\nHello from the `prime_checker_v2.rs` file.");

    if option == 1{
        check_one_number(num);
    }
    else if option == 2{
        check_till(num);
    }
    else {
        println!("Invalid option.")
    }
}