mod exceptions;
mod limits;
use limits::datatypes;
mod collections;
use collections::vectors;
use collections::strings;
use crate::collections::{hashmaps, exercises};

fn main() {

    // vectors::do_vector_stuff();
    //
    // strings::string_stuff();
    //
    // hashmaps::hash_it_up();
    let i = 13 / 4;
    println!("13 divided by 4 gives int: {}", i);

    exercises::exercise();

    // datatypes::ints();

    exceptions::do_exceptional_stuff();

}
