mod prepackages;
mod exceptions;
mod limits;
use limits::datatypes;
mod collections;
use collections::vectors;
use collections::strings;
use crate::collections::{hashmaps, exercises};

fn main() {

    // prepackages::gg::play_the_guessing_game();
    // prepackages::variables::variate();
    // prepackages::structs::structure();
    // prepackages::enums::enums::enumerate();

    // prepackages::ownership::own();

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
