#![allow(dead_code)]

use std::io;

use collections::strings;
use collections::vectors;

use crate::collections::{exercises, hashmaps};

mod prepackages;
mod exceptions;
mod limits;
mod collections;
mod generics;
mod fp;
mod pointers;

fn main() {

    loop {

        println!("What do you want to run? Currently, the options are: ");
        println!(" - a demonstration of all code from the Rust book (type \"da\" and hit enter)");
        println!(" - a demonstration of the latest chapter from the Rust book (dl)");
        println!(" - the new employee registration (in memory) system (er)");
        println!(" - all exercises that have been completed so far (ex)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        match input.trim() {
            "da" => demonstrate_all(),
            "dl" => demonstrate_latest(),
            "er" => exercises::employee_registration(),
            "ex" => exercise(),
            "exit" => break,
            _ => println!("That is not a valid option!")
        };
    }

}

fn demonstrate_all() {
    prepackages::gg::play_the_guessing_game();
    prepackages::variables::variate();
    prepackages::structs::structure();
    prepackages::enums::enums::enumerate();
    prepackages::ownership::own();
    vectors::do_vector_stuff();
    strings::string_stuff();
    hashmaps::hash_it_up();
    exceptions::do_exceptional_stuff();
    limits::ints();
    generics::functions::find_largests();
    generics::lifetimes::live_time();
    demonstrate_workout_generation();
}

fn demonstrate_workout_generation() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    fp::closures::generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn demonstrate_latest() {
    exercises::employee_registration();
}

fn exercise() {
    exercises::exercise();
}