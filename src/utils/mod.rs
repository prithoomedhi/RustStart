#![warn(missing_docs)]
#![allow(warnings)]
//! General purpose utility functions.

use colored::*;
use rand::{distributions::Uniform, Rng}; // 0.8.5

pub fn unique_elements_vector<
    T: std::ops::Add<Output = T> + std::fmt::Debug + std::cmp::PartialEq,
>(
    _list: Vec<T>,
) -> Vec<T> {
    //! Find all the UNIQUE elements in a given vector of Datatype T*
    //      * where T has the following attributes:
    //          Add()
    //          Debug()
    //          PartialEq()

    //  Arguments:
    //      _list: Vec<T>

    //  Returns:
    //      Vec<T>n
    let mut unique_list: Vec<T> = Vec::new();
    for item in _list {
        if !unique_list.contains(&item) {
            unique_list.push(item);
        }
    }
    return unique_list;
}

pub fn get_random_list(_min: i128, _max: i128, _length: usize) -> Vec<i128> {
    //! Generate a list of random numbers between _min and _max of length _length.
    //  Arguments:
    //      _min: i128
    //      _max: i128
    //      _length: usize
    //  Returns:
    //      Vec<i128>
    let _debug_statement = format!(
        "Generating a list of random numbers between {} and {} of length {}.",
        _min, _max, _length
    );
    println!("{}", _debug_statement.dimmed());
    let mut rng = rand::thread_rng();
    let range = Uniform::new(_min, _max);
    let mut list: Vec<i128> = Vec::new();
    for _ in 0.._length {
        list.push(rng.sample(range));
    }
    return list;
}

pub fn choose_random_element_from_vector<
    T: std::ops::Add<Output = T> + Copy + std::fmt::Debug + std::cmp::PartialEq,
>(
    _list: Vec<T>,
) -> T {
    //! Choose a random element from a given vector.
    //  Arguments:
    //      _list: Vec<T>
    //  Returns:
    //      T

    // println!("Choosing a random element from the given vector:\t{:?}.", _list);
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, _list.len() - 1);
    let index: usize = rng.sample(range);
    return _list[index];
}

pub fn random_integer(_min: i128, _max: i128) -> i128 {
    //! Generate a random integer between _min and _max.
    //  Arguments:
    //      _min: i128
    //      _max: i128
    //  Returns:
    //      i128
    let _debug_statement = format!(
        "INFO: Generating a random integer between {} and {}.",
        _min, _max
    );
    println!("{}", _debug_statement.dimmed());
    let mut rng = rand::thread_rng();
    let range = Uniform::new(_min, _max);
    return rng.sample(range);
}
