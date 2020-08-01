
pub fn ints() {

    // TODO: some casts, for instance:
    //  - from usize to i8/i32
    //  - from i32 to f32/f64
    //  - from i8/32 to char?
    //  - etc.

    let int_value = 67u8;
    // let int_value: i8 = 32;  // Will not compile: only unsigned (small/short) ints can be cast to chars (makes sense; e.g. what is -34 in UTF-8?)
    let char_value = int_value as char;

    println!("Integer value 67 as char: {}", char_value);
    println!("{} is still usable here, primitive values get copied when used as argument.", char_value);

    // usize 1 = size of one pointer/reference (dependent on machine architecture, 32 -> 4 bytes, 64 -> 8 bytes)
    let max_usize = usize::max_value();
    println!("Biggest number a usize variable can possibly hold: {}", max_usize);

    // println!("...probably won't fit in a char: {}", max_usize as char); // Can't even cast a usize to a char, really only u8
    let max_usize_short_unsigned = max_usize as u8;
    println!("Biggest possible usize value fits into an unsigned 8 bit int: {}...but looks like it's capped.", max_usize_short_unsigned);


}