use crate::collections::vectors;

pub fn exercise() {

    // TODO: Given a list of integers, use a vector and return the
    //  v mean (the average value),
    //  - median (when sorted, the value in the middle position)
    //  v mode (the value that occurs most often; a hash map will be helpful here) of the list.

    let random_vector = vectors::generate_random_vec(0, 15, 6);
    println!("Generated vector: {:?}", random_vector);
    println!("Mean: {}", vectors::find_mean(&random_vector));
    println!("Floating point mean: {}", vectors::find_floating_mean(&random_vector));

    let random_vector = vectors::generate_random_vec(0, 20, 30);
    println!("Generated vector: {:?}", random_vector);
    match vectors::find_mode(&random_vector) {
        Some(mode) => println!("Mode: {}", mode),
        None => println!("Mode could not be determined!")
    };


    // TODO: Convert strings to pig latin. The first consonant of each word is moved to the end
    //  of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a
    //  vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind
    //  the details about UTF-8 encoding!

    // TODO: Using a hash map and vectors, create a text interface to allow a user to add employee
    //  names to a department in a company. For example, “Add Sally to Engineering” or
    //  “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all
    //  people in the company by department, sorted alphabetically.

}

