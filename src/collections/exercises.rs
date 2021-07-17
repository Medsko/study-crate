use crate::collections::vectors;
use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::str::SplitWhitespace;

pub fn exercise() {

    // Given a list of integers, use a vector and return the
    //  - mean (the average value),
    //  - median (when sorted, the value in the middle position)
    //  - mode (the value that occurs most often; a hash map will be helpful here) of the list.

    let random_vector = vectors::generate_random_vec(0, 15, 6);
    println!("Generated vector: {:?}", random_vector);
    println!("Mean: {}", vectors::find_mean(&random_vector));
    println!("Floating point mean: {}", vectors::find_floating_mean(&random_vector));

    let random_vector = vectors::generate_random_vec(0, 20, 30);
    println!("Generated vector: {:?}", random_vector);
    println!("Median: {}", vectors::find_median(&random_vector));

    let random_vector = vectors::generate_random_vec(0, 20, 30);
    println!("Generated vector: {:?}", random_vector);
    match vectors::find_mode(&random_vector) {
        Some(mode) => println!("Mode: {}", mode),
        None => println!("Mode could not be determined!")
    };
}

// Using a hash map and vectors, create a text interface to allow a user to add employee
// names to a department in a company. For example, “Add Sally to Engineering” or
// “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all
// people in the company by department, sorted alphabetically.
pub fn employee_registration() {
    println!("Booting employee registration system...");
    println!("Operations should be in format: <Operation> (<Employee name> to/from <Department>)");

    let mut register: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut user_input = String::new();

        print!("Tell me what to do: ");
        io::stdout().flush().ok();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line!");

        let mut input_words = user_input.split_whitespace();

        let first_word = input_words.next().unwrap_or("");

        if "add".eq_ignore_ascii_case(first_word) {
            register_employee(&mut register, &mut input_words);
        } else if "print".eq_ignore_ascii_case(first_word) {
            print_register(&register);
        } else if "remove".eq_ignore_ascii_case(first_word) {
            remove_employee(&mut register, &mut input_words);
        } else if "exit".eq_ignore_ascii_case(first_word) {
            break;
        } else {
            panic!("Function '{}' not implemented!", first_word);
        }

    }
}

fn remove_employee(register: &mut HashMap<String, Vec<String>>, other_words: &mut SplitWhitespace) {
    // let employee_name = other_words.next().unwrap().to_string();
    // other_words.next(); // Skip the presumed 'from'
    let employee_name = match extract_full_name(other_words, "from") {
        Ok(name) => name,
        Err(msg) => panic!("{}", msg)
    };
    let department = other_words.next().unwrap().to_string();
    let mut department_employees = register.remove(&department).unwrap_or(Vec::new());

    if let Some(index) = department_employees.iter()
            .position(|element| element.eq_ignore_ascii_case(&employee_name)) {
        department_employees.remove(index);
    }

    if !department_employees.is_empty() {
        register.insert(department, department_employees);
    }
}

fn extract_full_name(words: &mut SplitWhitespace, stop: &str) -> Result<String, String> {
    let mut full_name = String::new();
    loop {
        if let Some(next) = words.next() {
            if stop.eq_ignore_ascii_case(next) {
                break;
            } else {
                full_name.push_str(next);
                full_name.push_str(" ");
            }
        } else {
            return Result::Err(format!("Keyword '{}' missing from input!", stop));
        }
    }

    Result::Ok(full_name.trim().to_string())
}

fn register_employee(register: &mut HashMap<String, Vec<String>>, other_words: &mut SplitWhitespace) {
    let employee_name = match extract_full_name(other_words, "to") {
        Ok(name) => name,
        Err(msg) => panic!("{}", msg)
    };
    let department = other_words.next().unwrap().to_string();
    let mut department_employees = register.remove(&department).unwrap_or(Vec::new());
    department_employees.push(employee_name);
    register.insert(department, department_employees);
}

fn print_register(register: &HashMap<String, Vec<String>>) {
    register.iter().for_each(|entry| {
        println!("Employees in {} department: ", entry.0);
        entry.1.iter().for_each(|name| println!("{}", name));
    });
}