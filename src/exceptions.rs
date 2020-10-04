use std::fs::{File};
use std::fs;
use std::io;
use std::io::Read;

pub fn do_exceptional_stuff() {
    // panic!("Crash and burn");

    let hello_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) =>  match error.kind() {
            io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(error) => panic!("Failed to create the file: {:?}", error)
            },
            other_error => panic!("Failed to open the file: {:?}", other_error)
        }
    };

    let directory = match fs::read_dir(".") {
        Ok(files) => files,
        Err(error) => panic!("Could not read the current directory!")
    };

    for file in directory {
        println!("{:?}", file);
    }

    // let file = File::open("asdf.txt").unwrap();
    // let file = File::open("asdf.txt").expect("Failed to open file asdf.txt!");

    match read_username_from_file("hello.txt") {
        Ok(user_name) => println!("User name read from file: {}", user_name),
        Err(error) => println!("Something horrible happened: {:?}!", error),
    }

    match succinct_read("hello.txt") {
        Ok(user_name) => println!("User name read from file: {}", user_name),
        Err(error) => println!("Something horrible happened: {:?}!", error),
    }

}

fn read_username_from_file(file: &str) -> Result<String, io::Error> {
//  a) let mut f = match File::open(file) {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };
    //
    // let mut s = String::new();
//  b) match f.read_to_string(&mut s) {
    //     Ok(size) => {
    //         println!("Bytes read: {}", size);
    //         Ok(s)
    //     },
    //     Err(error) => Err(error)
    // }
    let mut f = File::open(file)?;  // Does the same as a)
    let mut s = String::new();
    f.read_to_string(&mut s)?;  // These two lines are equal to b)
    Ok(s)
}

fn succinct_read(file: &str) -> Result<String, io::Error> {
    // let mut s = String::new();
    // File::open(file)?.read_to_string(&mut s)?;
    // Ok(s)
    fs::read_to_string(file)
}
